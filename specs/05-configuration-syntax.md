# 미꾸라지 DNS 설정 파일 명세 (Configuration Syntax Specification)

이 문서는 미꾸라지 DNS의 설정 파일 문법 및 구조를 정의합니다. 설정 파일의
문법은 직관적이고 확장성이 뛰어난 **CoreDNS의 Corefile** 구조에서 영감을
받아 설계되었습니다.

## 1. 개요

설정 파일은 미꾸라지 DNS Core 프로세스의 동작 방식(수신, 업스트림 라우팅,
캐싱, 플러그인 등)을 정의합니다. 블록(Block)과 지시어(Directive)를 기반으로
하여 전역 설정과 특정 도메인(Zone)에 대한 개별 설정을 유연하게 구성할 수
있습니다.

## 2. 기본 문법 구조 (Syntax Structure)

설정은 기본적으로 도메인 매칭 패턴과 그 블록 안에 포함된 지시어(플러그인)들로
구성됩니다.

```text
<매칭_패턴> {
    <지시어> <인자...>
    <지시어> {
        <하위_인자> <값>
    }
}
```

* **매칭 패턴 (Matching Pattern):** 특정 설정이 적용될 대상 도메인이나
  네트워크를 지정합니다. `.` (루트)를 사용하면 모든 쿼리에 대한 전역(Global)
  설정이 됩니다.
* **지시어 (Directive / Plugin):** 해당 블록에서 수행할 동작을 정의합니다.
  (예: `forward`, `cache`, `log` 등)
* **주석 (Comments):** `#` 기호로 시작하는 줄이나 문자는 주석으로 처리됩니다.

### 예시

```text
# 전역 설정 (모든 쿼리에 적용)
. {
    bind 127.0.0.1:53
    forward . tls://1.1.1.1 tls://8.8.8.8 {
        tls_servername cloudflare-dns.com
        health_check 5s
    }
    cache 3600
    log
    block_ads
}

# 조건부 포워딩 (사내망 도메인을 특정 내부 서버로 포워딩)
corp.local {
    forward . 192.168.1.10
}
```

## 3. 주요 지시어 (Core Directives)

아키텍처 명세(`00-architecture.md`)에 정의된 주요 기능들을 설정하기 위한
지시어들입니다.

### 3.1. `bind`

DNS 쿼리를 수신할 방식을 정의합니다. 포트 바인딩 외에도 각 OS별 트래픽 캡처
방식을 지정할 수 있습니다.

* `bind <ip>:<port>`: 로컬 서버 모드 (예: `bind 127.0.0.1:53`)
* `bind wfp`: Windows WFP 기반 캡처
* `bind wintun`: Windows Wintun 기반 캡처
* `bind ebpf`: Linux eBPF 기반 캡처
* `bind macos_extension`: macOS NetworkExtension 기반 캡처
* `bind ios_extension`: iOS NetworkExtension 기반 캡처
* `bind android_vpn`: Android VpnService 기반 캡처

### 3.2. `forward` (또는 `upstream`)

지정된 업스트림 서버로 DNS 쿼리를 포워딩합니다. 최신 프로토콜을 모두
지원합니다.

```text
forward <매칭> <목적지1> [<목적지2> ...] {
    <옵션>
}
```

* **프로토콜 지정:**
  * `udp://`, `tcp://`
  * `tls://` (DoT)
  * `https://` (DoH)
  * `quic://` (DoQ)
  * `odoh://` (Oblivious DoH)
* **옵션:**
  * `policy <round_robin|random|failover>`: 로드 밸런싱 정책
  * `health_check <시간>`: 헬스 체크 주기
  * `proxy <socks5://...|http://...>`: 프록시 경유 설정

### 3.3. `cache`

DNS 응답을 캐싱하여 처리 속도를 높입니다. 기본적으로 In-memory 캐시를
사용하지만, 필요에 따라 Redis, Memcached 등 외부 저장소를 설정할 수
있습니다.

```text
cache [TTL] {
    success <최대_건수>
    denial <최대_건수>
    
    # 외부 저장소 사용 시 설정 예시
    redis redis://127.0.0.1:6379
    # memcached memcache://127.0.0.1:11211
}
```

### 3.4. `split_dns` (OS 리졸버 우회)

Mudfish DNS의 트래픽 캡처를 우회하여 특정 도메인 쿼리를 시스템에 원래 설정되어 있던
기본(OS default) 리졸버로 보내도록 처리합니다. 내부망 전용 DNS가 별도로 구성되어 있어 캡처를
피해야 할 때 유용합니다.

```text
split_dns {
    domain *.local
    domain 192.168.in-addr.arpa
}
```

### 3.5. 로깅 및 분석 (`log`)

쿼리 히스토리 및 응답 통계를 기록합니다.

```text
log <형식> <경로> {
    format <json|text>
    ignore <도메인>
}
```

### 3.6. 플러그인 (Plugins)

플러그인별 커스텀 룰을 적용합니다.  Mudfish DNS 가 자체적으로 지원하는
built-in 플러그인 외에도, 사용자가 직접 만든 플러그인을 추가로 사용할 수 있습니다.
플러그인별 옵션은 각 플러그인의 문서를 참고하세요.

```text
<plugin_name> {
    <plugin_options>
}
```

## 4. 확장성 및 해석 (Parsing)

* 설정 파일의 파서는 `crates/common/src/config.rs` 등에 구현되며, 읽기 쉽고
  명확한 에러 메시지를 제공해야 합니다.
* 모듈화된 플러그인 아키텍처를 지원하기 위해, 알 수 없는 지시어라도 플러그인
  시스템을 통해 동적으로 해석될 수 있는 구조를 고려합니다.
* UI 프로세스는 이 설정 파일을 직접 수정하거나, IPC를 통해 설정 변경 명령을
  Core/Service 프로세스에 전달하여 설정 파일을 업데이트할 수 있어야 합니다.
