# 미꾸라지 DNS

이 문서에서는 미꾸라지 DNS 의 기본적인 아키텍처를 설명합니다. 미꾸라지 DNS 는 다양한 운영체제
(Windows, macOS, Linux, iOS 및 Android)의 기본(default) DNS resolver를 대체하기
위해 설계되었습니다.

프로그램이 실행되면 Background Service 로 동작하며, 사용자가 요청한 DNS 요청을 처리합니다.

## 기본적인 요소

미꾸라지 DNS는 기본적으로 데스크톱(Windows, Linux) 환경에서 다음과 같이 세 가지 독립적인
프로세스로 구성되며, 각 프로세스는 IPC(Inter-Process Communication)나 로컬 소켓 등을 통해
서로 안전하게 통신합니다.
단, 운영체제의 샌드박스 정책 및 모바일 환경의 특성(macOS, iOS, Android)에 따라 이 아키텍처는
단일 앱 번들 내장 또는 공유 라이브러리(Library) 형태로 변형되어 적용됩니다
(자세한 사항은 `01-packaging.md` 참조).

* Core Process
  * DNS 요청을 처리하고 라우팅하는 핵심 역할을 수행합니다.
* UI Process
  * 사용자 인터페이스를 제공하고 설정을 관리하며 사용자와 상호작용합니다.
* Service Process (Daemon)
  * OS 레벨에서 Core Process 의 생명주기(Start / Stop)를 제어하고,
    권한 상승이 필요한 네트워크 설정을 담당합니다.

### Core Process

* DNS Packet Redirection

  시스템에서 발생하거나 들어오는 DNS 패킷을 캡처하고 리다이렉션하는 역할을 합니다.  즉,
  "시스템 -> Mudfish DNS" 로 패킷이 전달되는 입구 역할을 합니다.

  지원하는 각 운영체제의 특성에 맞게 다음과 같은 다양한 방식을 지원해야 합니다.

  * 범용 (Local Server): Listen on 127.0.0.1:53
  * Windows: WFP-based mode, Wintun-based mode
  * macOS: NetworkExtension (DNSProxyProvider / NEDNSSettingsManager) 기반
  * Linux: eBPF 기반
  * iOS: NetworkExtension 기반
  * Android: VpnService (Local VPN mode) 기반 트래픽 캡처

* Upstream 프로토콜

  패킷을 받은 후, 지정된 Upstream DNS 로 포워딩을 실행할 때 어떠한 프로토콜을 사용할 것인지를
  지정하는 부분입니다. 최신 보안 프로토콜을 모두 포함합니다.

  * UDP (port 53)
  * TCP (port 53)
  * DoH (DNS over HTTPS)
  * DoT (DNS over TLS)
  * DoQ (DNS over QUIC)
  * ODoH (Oblivious DoH)
  * DNSSEC 지원
  * Custom Protocol 지원

* 안정성

  Upstream DNS 서버에 장애가 발생하더라도 안정적으로 DNS 요청을 처리하기 위한 구성 요소입니다.

  * Primary / Secondary DNS (Failover 지원)
  * Multiple Endpoints
  * Health Check
  * Load Balancing
  * Proxy 지원 기능 (SOCKS5, HTTP Proxy 경유 지원)
  * Captive Portal 탐지 및 예외 처리 지원

* 캐시 및 속도 (Caching & Speed)

  DNS 요청의 처리 속도를 높이고 업스트림 서버의 부하를 줄입니다.

  * Caching
    * 기본적으로 In-memory 캐시를 사용
    * Redis, Memcached 와 같은 외부 저장소(External storage)를 사용할 수
      있도록 설정 지원
    * Purging (캐시 초기화) 지원

* 라우팅 규칙 (Routing Rules)

  네트워크 환경에 맞는 효율적인 쿼리 분배 및 우회 처리를 수행합니다. 설정 상태는 UI를 통해 명확하게
  사용자가 확인하고 관리할 수 있어야 합니다.

  * 조건부 포워딩 및 라우팅 우회
    * 조건부 포워딩 (Conditional Forwarding): 사내망 등 특정 도메인 쿼리를 지정된
      내부 DNS 서버로 포워딩.
    * OS 리졸버 우회 (Bypass): `*.local`, `192.168.x.x`, `.corp` 등 로컬 네트워크
      쿼리가 미꾸라지 DNS 캡처를 우회하고 OS 기본 리졸버를 타도록 예외 처리.

* 로깅 및 통계 (Logging & Analytics)

  네트워크 활동을 분석하고 차단 효과를 사용자에게 시각적으로 보여주는 데이터 처리 부분입니다.

  * DNS 쿼리 히스토리 및 로그 기록 (개인정보 보호를 위해 On/Off 옵션 제공)
  * Upstream 서버별 실시간 응답 속도(Latency) 측정 및 모니터링
  * Plugin 관련
    * 광고 및 유해 도메인 차단 통계 분석

* Plugin 모델

  Plugin 모델을 도입하여 사용자가 직접 모듈(mod)을 만들어 사용할 수 있도록 하는 부분입니다.
  아래는 예시이며, Core plugin과 Custom plugin으로 나뉘어질 수 있습니다.

  Core plugin 은 Mudfish DNS 내부에서 기본적으로 제공하는 기능을 담당하는 플러그인인 반면,
  Custom plugin 은 사용자가 직접 작성하거나 제 3자가 작성한 모듈을 뜻합니다.

  * ACL
    * Include domains
    * Exclude domains (사용자 지정 화이트리스트/블랙리스트)

  * Official filtering
    * 광고 및 트래커 차단
    * Domain Category Service (유해 사이트, 악성코드 배포지 차단 등)

  * DNS Leaking Prevention (DNS 유출 방지 기능)

### UI Process

사용자 경험(UX)을 직접적으로 담당하는 프로세스로, 내부적으로 Core Process 혹은 Service
Process와 통신하여 제어 명령을 내립니다.

* 기본 기능
  * Start / Stop 버튼
  * 실시간 통계 및 쿼리 로그 대시보드 표시
  * 캐시 초기화(Purge) 인터페이스

* 기타 기능
  * 창을 닫을 때, 시스템 트레이(상태 표시줄)로 최소화
  * OS별 부팅 시 자동 실행 연동 (Windows, macOS, Linux 등)
  * 다크 모드 / 라이트 모드 등 테마 지원

### Service Process (Daemon)

UI Process 로부터 전달되는 명령어를 처리하고 높은 권한이 필요한 작업을 백그라운드에서 수행합니다.
각 OS의 백그라운드 서비스 시스템(Windows Service, Linux systemd 등)에 등록되어 동작합니다.
(단, macOS와 모바일 환경(iOS, Android)에서는 OS의 자체 프레임워크(NetworkExtension,
VpnService)가 Service의 역할을 대체하므로 독립된 데몬 프로세스를 사용하지 않습니다.)

* 주요 역할
  * Core Process 의 Start / Stop 및 비정상 종료 시 재시작(Watchdog) 처리
  * 시스템 DNS 설정 변경 및 가상 네트워크 인터페이스(Wintun 등) 생성 시 필요한 관리자 권한 대행

* 명령어 통신 (IPC)
  * UI Process 와 로컬 소켓 또는 Named Pipe 등을 통한 보안 통신
  * 지원 명령어: Start / Stop / Status / Restart
