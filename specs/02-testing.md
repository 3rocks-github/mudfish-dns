# Mudfish DNS 테스트 스위트 (Test Suite) 구성 명세

이 문서는 Mudfish DNS 프로젝트의 신뢰성을 보장하고 코드 품질을
유지하기 위한 테스트 스위트 구성 및 작성 규칙을 정의합니다. 

## 1. 테스트 기본 원칙
- **테스트 필수:** 코드를 변경하거나 새로운 기능을 추가할 때마다 반드시 해당
  변경 사항을 검증할 수 있는 테스트(단위 테스트 또는 통합 테스트)를 작성해야
  합니다.
- **테스트 자동화:** 모든 테스트는 `cargo test` 명령어로 한 번에 실행 및
  검증될 수 있어야 합니다.
- **격리 (Isolation):** 각 테스트는 서로 독립적으로 실행되어야 하며, 다른
  테스트의 결과나 상태에 의존해서는 안 됩니다. 전역 상태(Global State) 변경은
  지양하며, 필요할 경우 모의 객체(Mocking)를 적극적으로 활용합니다.

## 2. 테스트 종류 및 구성

### 2.1. 단위 테스트 (Unit Tests)

각 모듈 내부의 특정 기능(함수, 메서드, 구조체 등)이 의도한 대로 동작하는지
검증합니다.

- **위치:** 테스트 대상이 되는 코드와 동일한 파일 내 하단에 `tests` 모듈로
  작성합니다.
- **특징:**
  - 빠르고 가볍게 실행되어야 합니다.
  - 외부 리소스(네트워크, 파일 시스템 등)에 의존하지 않아야 합니다.
    (의존성이 필요한 경우 Mock을 사용합니다)
- **예시 (Rust):**
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_parse_dns_packet() {
          // ...
      }
  }
  ```

### 2.2. 통합 테스트 (Integration Tests)

여러 모듈 또는 크레이트(Crate)들이 함께 잘 동작하는지, 특히 프로세스 간
통신(IPC)이나 주요 비즈니스 로직의 흐름을 검증합니다.

- **위치:** 각 크레이트 루트(예: `crates/core/tests/`,
  `crates/common/tests/`) 아래에 위치시킵니다.
- **특징:**
  - 라이브러리의 공개 API만 테스트합니다.
  - `core` ↔ `service`, `core` ↔ `ui` 간의 IPC 메시지 송수신 및 처리
    흐름을 중점적으로 검증합니다.
  - 네트워크 통신이나 OS 데몬과의 상호 작용 등 실제 환경과 유사한 상황을
    가정하고 테스트할 수 있습니다.
  
### 2.3. End-to-End (E2E) 테스트

사용자 관점에서 애플리케이션의 전체 기능이 정상 작동하는지 검증합니다.
UI에서의 조작이 코어 및 서비스 프로세스를 거쳐 올바르게 반영되는지 확인합니다.

- **위치:** 워크스페이스 최상위의 `tests/` 디렉토리를 활용하거나 별도의
  E2E 프레임워크를 구성합니다.

## 3. 크레이트별 테스트 중점 사항

- **`crates/common`**: IPC 메시지 직렬화/역직렬화, 설정(Config) 파싱,
  공통 유틸리티의 정확성.
- **`crates/core`**: 업스트림 서버로의 라우팅 정책(UDP, TCP, DoH 등), DNS
  캐싱 동작, 안정성 제어(Failover, Load Balancing), 차단 플러그인 로직의
  단위 테스트.
- **`crates/service`**: OS별 네트워크 구성(Windows: WFP/Wintun, Linux:
  eBPF/iptables) 변경 명령 및 데몬 프로세스 수명 주기 제어 로직. (macOS 및
  모바일 환경의 NetworkExtension/VpnService 통합은 Service 프로세스를
  사용하지 않으므로, 각 플랫폼의 네이티브 앱 테스트 계층에서 검증합니다.)
- **`crates/ui`**: UI 상태 변경에 따른 올바른 IPC 명령 생성, 화면 컴포넌트
  데이터 렌더링 검증.

## 4. 다중 운영 체제 (Cross-Platform) 테스트 전략

Mudfish DNS는 Windows, macOS, Linux, iOS, Android를 지원하므로, 각 OS
환경에 맞는 테스트가 필수적입니다.

- **OS 특정 분기 테스트 (`#[cfg(target_os = "...")]`)**:
  플랫폼별로 다른 구현을 가지는 모듈은 각 대상 OS 환경에서 개별적으로 빌드 및
  테스트되어야 합니다. 로컬 개발 환경에서 지원하지 않는 OS의 테스트는 CI
  파이프라인(GitHub Actions 등)의 다중 플랫폼(Matrix) 빌드를 통해 커버합니다.
- **권한 제어 및 가상 환경**:
  - **Windows (WFP), Linux (eBPF), macOS (pf 등)**: 시스템 네트워크 설정
    변경은 관리자(Root) 권한이 필요하므로, 테스트 실행 시 권한이 없는
    경우(Non-admin) 테스트를 우회(Skip)하도록 처리하거나(`#[ignore]` 등)
    실행 시점에 권한을 확인하여 처리합니다. CI 환경에서는 격리된 가상
    머신(VM)에서 관리자 권한으로 실행합니다.
  - **모바일 (iOS / Android)**: 일반적인 데스크톱 기반 `cargo test` 명령으로는
    모바일 플랫폼 테스트가 제한적입니다. `cargo-dinghy`와 같은 크로스 테스트
    러너를 사용하거나, FFI(Foreign Function Interface)로 바인딩 후 각
    네이티브 앱 테스트 프레임워크(XCTest, Espresso) 내에서 통합 테스트를
    수행하는 전략을 채택해야 합니다.

## 5. 모의 객체 및 테스트 픽스처 활용 (Mocking & Fixtures)
- 의존성이 강한 부분(OS 시스템 호출, 실제 외부 DNS 서버 등)은 `mockall`과
  같은 Mock 라이브러리를 사용하거나 Trait를 분리하여 가짜(Fake) 객체로
  주입(Dependency Injection)하여 테스트합니다.
- 복잡한 설정 상태는 테스트 픽스처(Fixture) 또는 헬퍼 함수를 통해 반복되는
  코드(Boilerplate)를 최소화합니다.

## 6. 실행 방법 및 검증

- **모든 테스트 실행:**
  ```bash
  cargo test
  ```
- **특정 크레이트 테스트 실행:**
  ```bash
  cargo test -p mudfish-dns-core
  ```

CI/CD 파이프라인(예: GitHub Actions)에서 해당 명령이 성공적으로 통과해야만
코드가 병합(Merge)될 수 있도록 구성합니다.
