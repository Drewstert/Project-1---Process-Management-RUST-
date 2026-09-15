# Smart Traffic Management System: Code Breakdown

## How the Code Works

This program is a command-line application split into four distinct demonstrations of multi-threading:

* **Part 1 (Coordination):** Uses `Arc` (Atomic Reference Counted) and a `Mutex` to safely share a central `IntersectionState` among five threads. Four "sensor" threads write data to the state, and one "controller" thread reads it to make a traffic light decision.
* **Part 2A (Unsynchronized):** Simulates a race condition. Five threads try to increment a shared counter 100,000 times each. It intentionally separates the *load*, *add*, and *store* steps to create a logical flaw.
* **Part 2B (Synchronized):** Fixes the race condition by wrapping the counter in a `Mutex`, forcing threads to take turns modifying the value. 
* **Part 3 (Scheduling):** Drops down to system-level `libc` calls to pin threads to a single CPU core and alter their "nice" values (OS scheduling priorities), demonstrating how the Linux kernel distributes CPU time.

## Why the Team Chose This Approach

* **`Arc<Mutex<T>>` Pattern:** In Rust, data can only have one owner by default. `Arc` is used to give multiple threads shared ownership of the data, while `Mutex` provides "interior mutability"—allowing threads to safely mutate the data one at a time.
* **`Barrier` Synchronization:** The team used `Barrier::wait()` in the counting and scheduling demos to ensure all threads pause and then start their intensive work at the exact same millisecond. This forces maximum contention and makes race conditions or CPU competition obvious.

## What Could Go Wrong Without Synchronization

Without proper synchronization, the program suffers from **lost updates** (data races). 

In Part 2A, the code reads a value (e.g., 5), yields to another thread which also reads 5, and then both threads write `6` back to memory. Even though two vehicles passed, the counter only went up by one. In a real traffic system, this would lead to drastically undercounting vehicles and making incorrect signal decisions.

## What the Output Shows

* **State Dumps:** Part 1 prints a log of each sensor waking up, recording its data, and the final decision the controller makes based on that aggregated data.
* **Lost Vehicles:** Part 2A outputs an expected count of 500,000, but the actual count is much lower, explicitly printing the number of "lost vehicle detections."
* **Priority Impact:** Part 3 outputs the number of loop iterations each thread managed to complete in 3 seconds. It shows that threads with a lower `nice` value (higher priority) are granted more CPU time by Linux and therefore complete vastly more calculations than threads with higher `nice` values.

## Limitations of the Chosen Approach

* **Mutex Bottlenecks:** A `Mutex` forces threads to operate sequentially when accessing shared data. If the signal controller holds the lock for too long while calculating, all traffic sensors are blocked from recording new data, destroying the performance benefits of multi-threading.
* **Platform Dependency:** Part 3 relies heavily on `libc` and Linux-specific OS APIs (`sched_setaffinity`, `setpriority`). This code will fail to compile or crash on Windows or macOS.
* **Privilege Requirements:** Lowering a "nice" value to increase a thread's priority usually requires root (`sudo`) privileges in Linux. Running this as a normal user means the OS might reject the priority changes.
