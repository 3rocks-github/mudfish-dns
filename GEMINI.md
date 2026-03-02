# Mudfish DNS (미꾸라지 DNS)

## 프로젝트 개요
미꾸라지 DNS는 Windows, macOS, Linux, iOS 및 Android를 포함한 다양한 운영 체제에서 기본 DNS 리졸버를 대체하도록 설계된 DNS 리졸버입니다. 사용자의 DNS 요청을 안전하고 효율적으로 처리하기 위해 백그라운드 서비스로 작동합니다.

**주요 기술:**
- Rust (Cargo Workspace)
- 구성 요소 간의 안전한 통신을 위한 프로세스 간 통신 (IPC)
- 최신 DNS 업스트림 프로토콜: UDP, TCP, DoH (DNS over HTTPS), DoT (DNS over TLS), DoQ (DNS over QUIC), ODoH (Oblivious DoH), 그리고 DNSSEC.

## 아키텍처
애플리케이션은 관심사를 분리하고, 안정성을 향상시키며, 권한을 효과적으로 관리하기 위해 다중 프로세스 아키텍처로 구성됩니다. 작업 공간(Workspace)은 다음 크레이트들로 이루어져 있습니다:

1. **코어 프로세스 (`crates/core`)**: 애플리케이션의 핵심입니다. DNS 패킷 리디렉션(로컬 서버, WFP, Wintun, eBPF 등)을 처리하고, 쿼리를 업스트림 서버로 라우팅하며, 안정성(장애 조치, 로드 밸런싱, 프록시)을 관리하고, 캐싱 및 분할 DNS(Split DNS)를 처리하며, 플러그인(광고 차단이나 사용자 정의 규칙 등)을 실행합니다.
2. **UI 프로세스 (`crates/ui`)**: 사용자 인터페이스입니다. 실시간 통계, 시작/중지 제어, 구성 관리 및 시스템 트레이 통합을 위한 대시보드를 제공합니다. 명령을 실행하기 위해 코어 또는 서비스 프로세스와 통신합니다.
3. **서비스 프로세스 (`crates/service`)**: OS 수준에서 실행되는 데몬입니다 (예: Windows Service, macOS launchd (또는 NetworkExtension), Linux systemd). 코어 프로세스의 수명 주기(Watchdog)를 관리하고 관리자 권한이 필요한 네트워크 구성 변경을 처리합니다.
4. **공통 (`crates/common`)**: 다른 프로세스들 사이에서 공유되는 라이브러리, 구성 및 IPC 정의입니다.

## 빌드 및 실행
표준 Rust 작업 공간이므로 공통 개발 작업에 Cargo를 사용할 수 있습니다:

- **전체 작업 공간 빌드:**
  ```bash
  cargo build
  ```
- **구성 요소 실행:**
  - 코어: `cargo run --bin mudfish-dns-core`
  - UI: `cargo run --bin mudfish-dns-ui`
  - 서비스: `cargo run --bin mudfish-dns-service`
- **테스트 실행:**
  ```bash
  cargo test
  ```

*(참고: OS 및 사용된 트래픽 가로채기 방식에 따라 코어 및 서비스 프로세스를 실행하려면 상승된 관리자 권한이 필요할 수 있습니다.)*

## 개발 규칙
- **명세 (Specifications):** 항상 `./specs/` 디렉토리 아래에 있는 `*.md` 파일들의 내용이 Mudfish DNS의 공식 명세(Specification)이므로, 구현 시 반드시 해당 내용을 참고하고 반영해야 합니다.
- **언어:** 이 프로젝트는 **Rust**로 개발됩니다.
- **코딩 스타일:** 사용자 기본 설정에 따라 코드에 불필요한 주석을 남기지 마세요.
- **모듈화:** 아키텍처에 명시된 엄격한 관심사 분리를 준수하세요. 공유 로직에는 `crates/common`을 사용하세요.
- **통신:** 프로세스는 공유 상태를 가정하는 대신 정의된 IPC 인터페이스를 사용하여 안전하게 통신해야 합니다.
- **테스트 (Test):** 코드를 변경할 때마다 단위 테스트(Unit Test) 및 통합 테스트(Integration Test)를 작성하여 해당 변경 사항이 의도대로 올바르게 동작하는지 반드시 검증해야 합니다. 자세한 테스트 방법은 `specs/02-testing.md` 명세를 참고하세요.
