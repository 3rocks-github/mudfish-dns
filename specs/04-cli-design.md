# 미꾸라지 DNS CLI 명세 (CLI Specification)

이 문서는 미꾸라지 DNS를 명령줄(Command Line)에서 제어하고 관리하기 위한 CLI 도구
(`mudfish-dns-cli`)의 설계 및 명령어 체계를 정의합니다. `crates/cli` 프로세스는 이 명세를
바탕으로 구현되어야 합니다.

## 1. 개요
CLI는 헤드리스(Headless) 환경, 스크립트 자동화, 또는 터미널(콘솔)을 선호하는 사용자를 위해
제공됩니다. 백그라운드에서 실행 중인 Core Process 및 Service Process와 IPC를 통해 통신하여
제어 명령을 전달하고 상태를 확인합니다.

## 2. 기본 명령어 구조
명령어는 `mudfish-dns` (또는 `mudfish-dns-cli`) 바이너리를 통해 실행되며, 서브커맨드
(Subcommands) 방식을 채택합니다. 일반적으로 `clap`과 같은 라이브러리를 사용하여 구성합니다.

```bash
mudfish-dns [OPTIONS] <SUBCOMMAND>
```

## 3. 주요 명령어 (Subcommands)

### 3.1. 상태 제어 및 확인
* `start`: 미꾸라지 DNS 보호를 시작합니다. (Service를 통해 Core 프로세스 구동)
* `stop`: 미꾸라지 DNS 보호를 중지합니다.
* `restart`: 미꾸라지 DNS 보호를 재시작합니다.
* `status`: 현재 실행 여부(보호 중/중지됨), 사용 중인 업스트림 서버, 기본 통계를 터미널에
  출력합니다.

### 3.2. 관리 및 유지보수
* `purge-cache`: 코어 내부의 DNS 캐시를 즉시 초기화(Purge)합니다.
* `logs`: 실시간으로 처리되는 DNS 쿼리 로그 및 차단 로그를 터미널에 출력합니다.
  (Ctrl+C로 종료 전까지 계속 출력)

### 3.3. 설정 관리 (Config)
* `config show`: 현재 적용된 시스템 및 동작 설정을 화면에 출력합니다.
* `config set <KEY> <VALUE>`: 특정 설정 값을 변경하고 런타임에 즉시 반영을 시도합니다.
  * 예: `mudfish-dns config set upstream_protocol doh`

## 4. 아키텍처 및 권한
* **통신 방식:** CLI는 UI 프로세스와 동일하게 `crates/common/src/ipc.rs`에 정의된 IPC
  메시지를 사용하여 데몬(Service/Core)과 통신합니다.
* **실행 권한:** 일반 사용자 권한으로 실행되며, 관리자 권한이 필요한 제어는 Service 프로세스에
  위임합니다.
