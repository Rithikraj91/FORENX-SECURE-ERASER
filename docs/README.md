# ForenX Secure Eraser - Documentation



## 1. Scope



This workspace contains the Layer 3 erasure components assigned to ForenX:



- Secure Drive Eraser

- Secure File & Metadata Eraser



Advanced File Recovery is outside the scope of this workspace.



The current implementation focuses on safe discovery, capability assessment,

sanitization method selection, safety validation, mock execution, verification,

and Tauri integration.



---



## 2. Current Implementation Status



### Secure Drive Eraser



Implemented:



- Drive discovery

- Drive identity resolution

- Conservative media classification

- Capability assessment

- Sanitization method selection

- Safety validation

- Mock erasure engine

- Mock verification

- Tauri integration



### Secure File & Metadata Eraser



Implemented:



- File and directory target resolution

- Filesystem probing

- Reparse-point boundary handling

- File identity capture

- Capability assessment

- Sanitization method selection

- Safety validation

- Mock erasure engine

- Mock verification

- Tauri integration



---



## 3. Safety Model



The implementation follows a fail-closed approach.



Execution must not proceed when critical safety information is unavailable

or when the target is classified as unsafe.



Examples of safety conditions that can block execution include:



- System drive or protected system path

- Boot drive

- Read-only target

- Unknown critical safety information

- Unsafe or unknown reparse-point boundaries

- Invalid sanitization plan

- No executable sanitization methods



Safety decisions are enforced in the Rust core and are not trusted to the

React frontend.



---



## 4. Mock Execution



The current erasure engines are simulation-only.



### Drive Mock Engine



The Drive mock engine:



- Does not access physical drives

- Does not issue destructive device commands

- Does not overwrite sectors

- Produces deterministic simulated progress

- Produces a simulated verification token



### File Mock Engine



The File mock engine:



- Does not delete files

- Does not overwrite file contents

- Does not modify metadata

- Does not modify alternate data streams

- Does not modify slack space

- Does not modify free space

- Produces deterministic simulated progress

- Produces a simulated verification token



The `is\_simulated\_mock` field is used to make this state explicit.



---



## 5. Verification Limitations



Current verification verifies the result of the mock execution.



A successful mock verification means that the expected simulation data,

methods, target, execution state, and verification token matched.



It does NOT prove:



- Physical magnetic media sanitization

- Physical NAND sanitization

- Successful SSD/NVMe garbage-collection cleanup

- Removal of data from remapped sectors

- Removal of data from hidden device areas

- Destruction of physical storage media



Real physical sanitization requires a separate destructive backend and

additional hardware-specific validation.



---



## 6. Storage Technology Limitations



### HDD



Host-side overwrite may be an applicable sanitization technique depending

on the device and sanitization requirements.



### SSD / NVMe



Host-side file or logical-block overwrites cannot by themselves guarantee

physical NAND sanitization because storage controllers may use mechanisms

such as wear leveling, garbage collection, and logical-to-physical

mapping.



Appropriate device-level sanitization or cryptographic erase mechanisms

should be preferred when supported and appropriate.



### Cryptographic Erase



Cryptographic erase may be used when the device's encryption and key

management properties provide sufficient assurance for the applicable

sanitization requirement.



---



## 7. NIST SP 800-88 Alignment



The design is intended to be aligned with NIST SP 800-88 Rev. 2 and

applicable storage sanitization standards.



This project must NOT claim to be NIST certified.



Sanitization method selection is based on available evidence and device

capabilities rather than assuming a fixed number of overwrite passes.



Historical multi-pass schemes must not be represented as a universal

NIST requirement.



---



## 8. Testing



The workspace contains unit and integration tests covering:



- Drive capability assessment

- Drive classification

- Drive identity

- Drive method selection

- Drive safety validation

- Drive mock discovery

- Drive mock execution

- Drive verification

- File capability assessment

- File method selection

- File safety validation

- File mock execution

- File verification

- File target resolution

- Filesystem probing

- Reparse-point handling

- Inaccessible targets

- Duplicate targets

- File identity handling

- Alternate data stream probing



The workspace test suite must pass before changes are considered ready

for integration.



---



## 9. Tauri Integration



The Tauri application uses Tauri v2.



The current frontend provides integration-test controls for:



- Mock Drive Eraser

- Mock File Eraser



The frontend invokes Rust commands through Tauri IPC.



The frontend is not responsible for making authoritative safety decisions.



The Rust core performs the safety validation before mock execution.



---



## 10. Real Destructive Backend



A real destructive erasure backend is intentionally not enabled in the

current development implementation.



Before implementing or enabling destructive operations, the project

requires:



1\. Detailed safety review

2\. Hardware-specific capability validation

3\. Disposable test hardware or isolated virtual test storage

4\. Explicit system/boot-device protection

5\. Independent target revalidation

6\. Cancellation and failure handling

7\. Verification appropriate to the selected sanitization method

8\. Audit logging

9\. Integration testing

10\. Review of applicable standards and hardware documentation



No destructive testing should be performed against the host operating

system disk or important user data.



---



## 11. Current Development Boundary



Current status:



```text

Discovery

&#x20;   â†“

Capability Assessment

&#x20;   â†“

Method Selection

&#x20;   â†“

Safety Validation

&#x20;   â†“

Mock Execution

&#x20;   â†“

Mock Verification

&#x20;   â†“

Tauri Integration

&#x20;   â†“

Testing



