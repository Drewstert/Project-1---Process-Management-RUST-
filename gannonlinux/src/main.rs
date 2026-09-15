use std::env;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Barrier, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================
// SMART TRAFFIC MANAGEMENT SYSTEM
// ============================================================

// ============================================================
// PART 1: FIVE DISTINCT THREADS
// ============================================================

fn demo_thread_creation() {
    println!("\n==================================================");
    println!("SMART TRAFFIC MANAGEMENT SYSTEM");
    println!("PART 1: THREAD CREATION");
    println!("==================================================");

    let mut handles = Vec::new();

    // --------------------------------------------------------
    // Thread 1: North/South traffic sensor
    // --------------------------------------------------------
    handles.push(
        thread::Builder::new()
            .name("north-south-sensor".to_string())
            .spawn(|| {
                println!("[North/South Sensor] STARTED");

                let vehicles_detected = 42;

                thread::sleep(Duration::from_millis(150));

                println!(
                    "[North/South Sensor] Work: detected {} vehicles",
                    vehicles_detected
                );

                println!("[North/South Sensor] FINISHED");
            })
            .expect("Failed to create North/South sensor"),
    );

    // --------------------------------------------------------
    // Thread 2: East/West traffic sensor
    // --------------------------------------------------------
    handles.push(
        thread::Builder::new()
            .name("east-west-sensor".to_string())
            .spawn(|| {
                println!("[East/West Sensor] STARTED");

                let vehicles_detected = 35;

                thread::sleep(Duration::from_millis(100));

                println!(
                    "[East/West Sensor] Work: detected {} vehicles",
                    vehicles_detected
                );

                println!("[East/West Sensor] FINISHED");
            })
            .expect("Failed to create East/West sensor"),
    );

    // --------------------------------------------------------
    // Thread 3: Pedestrian monitor
    // --------------------------------------------------------
    handles.push(
        thread::Builder::new()
            .name("pedestrian-monitor".to_string())
            .spawn(|| {
                println!("[Pedestrian Monitor] STARTED");

                let pedestrians_waiting = 8;

                thread::sleep(Duration::from_millis(120));

                println!(
                    "[Pedestrian Monitor] Work: {} pedestrians waiting to cross",
                    pedestrians_waiting
                );

                println!("[Pedestrian Monitor] FINISHED");
            })
            .expect("Failed to create pedestrian monitor"),
    );

    // --------------------------------------------------------
    // Thread 4: Emergency vehicle detector
    // --------------------------------------------------------
    handles.push(
        thread::Builder::new()
            .name("emergency-detector".to_string())
            .spawn(|| {
                println!("[Emergency Detector] STARTED");

                let emergency_vehicle_detected = true;

                thread::sleep(Duration::from_millis(80));

                if emergency_vehicle_detected {
                    println!(
                        "[Emergency Detector] Work: emergency vehicle detected"
                    );
                } else {
                    println!(
                        "[Emergency Detector] Work: no emergency vehicle detected"
                    );
                }

                println!("[Emergency Detector] FINISHED");
            })
            .expect("Failed to create emergency detector"),
    );

    // --------------------------------------------------------
    // Thread 5: Traffic signal controller
    // --------------------------------------------------------
    handles.push(
        thread::Builder::new()
            .name("signal-controller".to_string())
            .spawn(|| {
                println!("[Signal Controller] STARTED");

                let green_light_seconds = 30;

                thread::sleep(Duration::from_millis(180));

                println!(
                    "[Signal Controller] Work: green light timer set to {} seconds",
                    green_light_seconds
                );

                println!("[Signal Controller] FINISHED");
            })
            .expect("Failed to create signal controller"),
    );

    // Wait until all five threads finish.
    for handle in handles {
        handle.join().expect("A traffic-system thread panicked");
    }

    println!();
    println!("Main system: all five traffic-control threads finished.");
}

// ============================================================
// PART 2A: UNSYNCHRONIZED VEHICLE COUNTER
// ============================================================

fn demo_unsynchronized() {
    println!("\n==================================================");
    println!("PART 2A: UNSYNCHRONIZED TRAFFIC COUNTER");
    println!("==================================================");

    const SENSOR_COUNT: usize = 5;
    const VEHICLES_PER_SENSOR: usize = 100_000;

    println!(
        "Five road sensors are updating the same total vehicle counter."
    );

    println!(
        "Each sensor reports {} vehicle detections.",
        VEHICLES_PER_SENSOR
    );

    /*
        We use AtomicUsize so individual memory accesses are safe.

        However, the complete operation:

            1. load the current value
            2. add one
            3. store the new value

        is NOT performed atomically.

        Two sensors can read the same old value and both write
        the same new value. One vehicle detection is then lost.
    */

    let total_vehicles =
        Arc::new(AtomicUsize::new(0));

    let starting_barrier =
        Arc::new(Barrier::new(SENSOR_COUNT));

    let mut handles = Vec::new();

    for sensor_id in 1..=SENSOR_COUNT {
        let counter =
            Arc::clone(&total_vehicles);

        let barrier =
            Arc::clone(&starting_barrier);

        let name =
            format!("road-sensor-{sensor_id}");

        let handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                println!("[{name}] STARTED");

                // All sensors begin counting at approximately
                // the same time.
                barrier.wait();

                for detection in 0..VEHICLES_PER_SENSOR {
                    let old_value =
                        counter.load(Ordering::Relaxed);

                    // Encourage thread interleaving.
                    if detection % 50 == 0 {
                        thread::yield_now();
                    }

                    counter.store(
                        old_value + 1,
                        Ordering::Relaxed,
                    );
                }

                println!(
                    "[{name}] Work: reported {} vehicles",
                    VEHICLES_PER_SENSOR
                );

                println!("[{name}] FINISHED");
            })
            .expect("Failed to create road sensor");

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let expected =
        SENSOR_COUNT * VEHICLES_PER_SENSOR;

    let actual =
        total_vehicles.load(Ordering::Relaxed);

    println!();
    println!("Expected total vehicles : {expected}");
    println!("Recorded total vehicles : {actual}");

    if actual <= expected {
        println!(
            "Lost vehicle detections : {}",
            expected - actual
        );
    }

    if actual != expected {
        println!();
        println!(
            "RESULT: The traffic count is incorrect because updates were lost."
        );

        println!(
            "This demonstrates an unsynchronized logical race."
        );
    } else {
        println!();
        println!(
            "RESULT: This run happened to produce the correct count."
        );

        println!(
            "Run it again because thread scheduling is nondeterministic."
        );
    }
}

// ============================================================
// PART 2B: SYNCHRONIZED VEHICLE COUNTER
// ============================================================

fn demo_synchronized() {
    println!("\n==================================================");
    println!("PART 2B: SYNCHRONIZED TRAFFIC COUNTER");
    println!("==================================================");

    const SENSOR_COUNT: usize = 5;
    const VEHICLES_PER_SENSOR: usize = 100_000;

    println!(
        "The same five sensors now update a Mutex-protected counter."
    );

    /*
        Arc allows all sensor threads to share ownership.

        Mutex allows only one sensor at a time to modify the
        vehicle count.
    */

    let total_vehicles =
        Arc::new(Mutex::new(0usize));

    let starting_barrier =
        Arc::new(Barrier::new(SENSOR_COUNT));

    let mut handles = Vec::new();

    for sensor_id in 1..=SENSOR_COUNT {
        let counter =
            Arc::clone(&total_vehicles);

        let barrier =
            Arc::clone(&starting_barrier);

        let name =
            format!("protected-road-sensor-{sensor_id}");

        let handle = thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                println!("[{name}] STARTED");

                barrier.wait();

                for _ in 0..VEHICLES_PER_SENSOR {
                    /*
                        Only one thread can hold the MutexGuard
                        at a time.
                    */
                    let mut vehicle_count =
                        counter.lock().unwrap();

                    *vehicle_count += 1;

                    /*
                        vehicle_count goes out of scope at the end
                        of each loop iteration, releasing the mutex.
                    */
                }

                println!(
                    "[{name}] Work: safely recorded {} vehicles",
                    VEHICLES_PER_SENSOR
                );

                println!("[{name}] FINISHED");
            })
            .expect("Failed to create protected road sensor");

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let expected =
        SENSOR_COUNT * VEHICLES_PER_SENSOR;

    let actual =
        *total_vehicles.lock().unwrap();

    println!();
    println!("Expected total vehicles : {expected}");
    println!("Recorded total vehicles : {actual}");

    if actual == expected {
        println!();
        println!(
            "RESULT: Correct count. The Mutex prevented lost updates."
        );
    }

    println!();
    println!(
        "The Mutex provides MUTUAL EXCLUSION."
    );

    println!(
        "It does NOT guarantee which sensor thread runs first."
    );
}

// ============================================================
// PART 3: LINUX SCHEDULING INVESTIGATION
// ============================================================

#[derive(Debug)]
struct SchedulingResult {
    task_name: String,
    requested_nice: i32,
    actual_nice: i32,
    policy: String,
    static_priority: i32,
    work_completed: u64,
}

fn demo_scheduling() {
    println!("\n==================================================");
    println!("PART 3: TRAFFIC ANALYSIS SCHEDULING");
    println!("==================================================");

    println!(
        "Three traffic-analysis threads will compete for CPU time."
    );

    println!(
        "Linux nice values will be modified to investigate scheduling."
    );

    let starting_nice =
        current_nice();

    println!();
    println!(
        "Default main-thread nice value: {starting_nice}"
    );

    let selected_cpu =
        first_allowed_cpu();

    match selected_cpu {
        Some(cpu_number) => {
            println!(
                "All analysis threads will compete on CPU {cpu_number}."
            );
        }

        None => {
            println!(
                "Could not determine CPU affinity."
            );
        }
    }

    /*
        Three real-world traffic-analysis jobs:

        1. Emergency traffic analysis
        2. Congestion analysis
        3. Historical statistics analysis

        The Linux nice value is changed for each worker.

        Lower nice number = more favorable scheduling.
        Higher nice number = less favorable scheduling.

        This does NOT guarantee execution order.
    */

    let tasks = [
        (
            "Emergency Traffic Analysis",
            starting_nice,
        ),
        (
            "Congestion Analysis",
            (starting_nice + 5).min(19),
        ),
        (
            "Historical Statistics",
            (starting_nice + 10).min(19),
        ),
    ];

    let barrier =
        Arc::new(Barrier::new(3));

    let mut handles = Vec::new();

    for (
        task_name,
        requested_nice,
    ) in tasks
    {
        let barrier =
            Arc::clone(&barrier);

        let cpu =
            selected_cpu;

        let task_name =
            task_name.to_string();

        let handle = thread::Builder::new()
            .name(task_name.clone())
            .spawn(move || {
                println!();
                println!(
                    "[{task_name}] STARTED"
                );

                println!(
                    "[{task_name}] Requested nice value: {requested_nice}"
                );

                // Put all three workers on one CPU so they
                // directly compete for processing time.
                if let Some(cpu_number) = cpu {
                    if !pin_current_thread_to_cpu(
                        cpu_number,
                    ) {
                        println!(
                            "[{task_name}] Warning: CPU affinity failed."
                        );
                    }
                }

                if !set_current_thread_nice(
                    requested_nice,
                ) {
                    println!(
                        "[{task_name}] Warning: could not change nice value: {}",
                        std::io::Error::last_os_error()
                    );
                }

                let actual_nice =
                    current_nice();

                let (
                    policy,
                    static_priority,
                ) =
                    current_scheduling_information();

                println!(
                    "[{task_name}] Linux policy = {policy}"
                );

                println!(
                    "[{task_name}] Static priority = {static_priority}"
                );

                println!(
                    "[{task_name}] Actual nice value = {actual_nice}"
                );

                // All three begin their CPU-intensive work together.
                barrier.wait();

                let start =
                    Instant::now();

                let test_duration =
                    Duration::from_secs(3);

                let mut calculations: u64 = 0;

                /*
                    Simulates CPU-intensive traffic-data analysis.
                */
                while start.elapsed() < test_duration {
                    calculations =
                        calculations.wrapping_add(1);

                    std::hint::black_box(
                        calculations,
                    );
                }

                println!(
                    "[{task_name}] FINISHED with {calculations} calculations"
                );

                SchedulingResult {
                    task_name,
                    requested_nice,
                    actual_nice,
                    policy,
                    static_priority,
                    work_completed:
                        calculations,
                }
            })
            .expect(
                "Failed to create scheduling thread",
            );

        handles.push(handle);
    }

    let mut results =
        Vec::new();

    for handle in handles {
        results.push(
            handle.join().unwrap(),
        );
    }

    results.sort_by_key(
        |result| result.actual_nice,
    );

    println!();
    println!(
        "================ SCHEDULING RESULTS ================"
    );

    for result in &results {
        println!();
        println!(
            "Task: {}",
            result.task_name
        );

        println!(
            "Requested nice: {}",
            result.requested_nice
        );

        println!(
            "Actual nice: {}",
            result.actual_nice
        );

        println!(
            "Scheduling policy: {}",
            result.policy
        );

        println!(
            "Static priority: {}",
            result.static_priority
        );

        println!(
            "Calculations completed: {}",
            result.work_completed
        );
    }

   /* println!();
    println!("INTERPRETATION:");

    println!(
        "- Linux normally uses SCHED_OTHER for these threads."
    );

    println!(
        "- Lower numeric nice values receive more favorable scheduling."
    );

    println!(
        "- Higher numeric nice values receive less favorable scheduling."
    );

    println!(
        "- Nice values influence scheduling but do NOT guarantee execution order."
    );

    println!(
        "- Results can vary between runs because OS scheduling is nondeterministic."
    );
    */
}

// ============================================================
// LINUX FUNCTIONS USED FOR SCHEDULING DEMONSTRATION
// ============================================================

fn current_nice() -> i32 {
    unsafe {
        libc::getpriority(
            libc::PRIO_PROCESS,
            0,
        )
    }
}

fn set_current_thread_nice(
    nice_value: i32,
) -> bool {
    let result = unsafe {
        libc::setpriority(
            libc::PRIO_PROCESS,
            0,
            nice_value,
        )
    };

    result == 0
}

fn current_scheduling_information()
    -> (String, i32)
{
    unsafe {
        let current_thread =
            libc::pthread_self();

        let mut policy:
            libc::c_int = 0;

        let mut parameters:
            libc::sched_param =
            std::mem::zeroed();

        let result =
            libc::pthread_getschedparam(
                current_thread,
                &mut policy,
                &mut parameters,
            );

        if result != 0 {
            return (
                format!(
                    "UNKNOWN(error={result})"
                ),
                -1,
            );
        }

        let policy_name =
            match policy {
                libc::SCHED_OTHER =>
                    "SCHED_OTHER",

                libc::SCHED_FIFO =>
                    "SCHED_FIFO",

                libc::SCHED_RR =>
                    "SCHED_RR",

                libc::SCHED_BATCH =>
                    "SCHED_BATCH",

                libc::SCHED_IDLE =>
                    "SCHED_IDLE",

                _ =>
                    "UNKNOWN",
            };

        (
            policy_name.to_string(),
            parameters.sched_priority,
        )
    }
}

fn first_allowed_cpu()
    -> Option<usize>
{
    unsafe {
        let mut cpu_set:
            libc::cpu_set_t =
            std::mem::zeroed();

        libc::CPU_ZERO(
            &mut cpu_set,
        );

        let result =
            libc::sched_getaffinity(
                0,
                std::mem::size_of::<
                    libc::cpu_set_t
                >(),
                &mut cpu_set,
            );

        if result != 0 {
            return None;
        }

        for cpu in
            0..libc::CPU_SETSIZE as usize
        {
            if libc::CPU_ISSET(
                cpu,
                &cpu_set,
            ) {
                return Some(cpu);
            }
        }

        None
    }
}

fn pin_current_thread_to_cpu(
    cpu: usize,
) -> bool {
    unsafe {
        let mut cpu_set:
            libc::cpu_set_t =
            std::mem::zeroed();

        libc::CPU_ZERO(
            &mut cpu_set,
        );

        libc::CPU_SET(
            cpu,
            &mut cpu_set,
        );

        libc::sched_setaffinity(
            0,
            std::mem::size_of::<
                libc::cpu_set_t
            >(),
            &cpu_set,
        ) == 0
    }
}

// ============================================================
// MAIN MENU
// ============================================================

fn print_usage(
    program_name: &str,
) {
    println!();
    println!("SMART TRAFFIC MANAGEMENT SYSTEM");

    println!();
    println!("Usage:");

    println!(
        "{program_name} creation"
    );

    println!(
        "{program_name} unsync"
    );

    println!(
        "{program_name} sync"
    );

    println!(
        "{program_name} schedule"
    );

    println!(
        "{program_name} all"
    );
}

fn main() {
    let args: Vec<String> =
        env::args().collect();

    if args.len() != 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "creation" => {
            demo_thread_creation();
        }

        "unsync" => {
            demo_unsynchronized();
        }

        "sync" => {
            demo_synchronized();
        }

        "schedule" => {
            demo_scheduling();
        }

        "all" => {
            demo_thread_creation();
            demo_unsynchronized();
            demo_synchronized();
            demo_scheduling();
        }

        _ => {
            print_usage(&args[0]);
        }
    }
}