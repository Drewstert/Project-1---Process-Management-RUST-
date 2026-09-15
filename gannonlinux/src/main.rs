use std::env;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Barrier, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================
// SHARED INTERSECTION STATE
// ============================================================

#[derive(Debug)]
struct IntersectionState {
    north_south_vehicles: u32,
    east_west_vehicles: u32,
    pedestrians_waiting: u32,
    emergency_vehicle_detected: bool,
    signal_decision: String,
}

// ============================================================
// PART 1: FIVE COORDINATED THREADS
// ============================================================

fn demo_thread_creation() {
    println!("\n==================================================");
    println!("SMART TRAFFIC MANAGEMENT SYSTEM");
    println!("PART 1: COORDINATED THREAD CREATION");
    println!("==================================================");

    let state = Arc::new(Mutex::new(IntersectionState {
        north_south_vehicles: 0,
        east_west_vehicles: 0,
        pedestrians_waiting: 0,
        emergency_vehicle_detected: false,
        signal_decision: "No decision yet".to_string(),
    }));

    let mut handles = Vec::new();

    // ========================================================
    // THREAD 1: NORTH/SOUTH TRAFFIC SENSOR
    // ========================================================

    {
        let shared_state = Arc::clone(&state);

        handles.push(
            thread::Builder::new()
                .name("north-south-sensor".to_string())
                .spawn(move || {
                    println!("[North/South Sensor] STARTED");

                    thread::sleep(Duration::from_millis(100));

                    let detected = 42;

                    {
                        let mut state = shared_state.lock().unwrap();

                        state.north_south_vehicles = detected;
                    }

                    println!(
                        "[North/South Sensor] Work: detected {detected} vehicles"
                    );

                    println!("[North/South Sensor] FINISHED");
                })
                .expect("Failed to create North/South sensor"),
        );
    }

    // ========================================================
    // THREAD 2: EAST/WEST TRAFFIC SENSOR
    // ========================================================

    {
        let shared_state = Arc::clone(&state);

        handles.push(
            thread::Builder::new()
                .name("east-west-sensor".to_string())
                .spawn(move || {
                    println!("[East/West Sensor] STARTED");

                    thread::sleep(Duration::from_millis(120));

                    let detected = 27;

                    {
                        let mut state = shared_state.lock().unwrap();

                        state.east_west_vehicles = detected;
                    }

                    println!(
                        "[East/West Sensor] Work: detected {detected} vehicles"
                    );

                    println!("[East/West Sensor] FINISHED");
                })
                .expect("Failed to create East/West sensor"),
        );
    }

    // ========================================================
    // THREAD 3: PEDESTRIAN MONITOR
    // ========================================================

    {
        let shared_state = Arc::clone(&state);

        handles.push(
            thread::Builder::new()
                .name("pedestrian-monitor".to_string())
                .spawn(move || {
                    println!("[Pedestrian Monitor] STARTED");

                    thread::sleep(Duration::from_millis(80));

                    let pedestrians = 6;

                    {
                        let mut state = shared_state.lock().unwrap();

                        state.pedestrians_waiting = pedestrians;
                    }

                    println!(
                        "[Pedestrian Monitor] Work: {pedestrians} pedestrians waiting"
                    );

                    println!("[Pedestrian Monitor] FINISHED");
                })
                .expect("Failed to create pedestrian monitor"),
        );
    }

    // ========================================================
    // THREAD 4: EMERGENCY VEHICLE DETECTOR
    // ========================================================

    {
        let shared_state = Arc::clone(&state);

        handles.push(
            thread::Builder::new()
                .name("emergency-detector".to_string())
                .spawn(move || {
                    println!("[Emergency Detector] STARTED");

                    thread::sleep(Duration::from_millis(150));

                    let emergency_detected = true;

                    {
                        let mut state = shared_state.lock().unwrap();

                        state.emergency_vehicle_detected =
                            emergency_detected;
                    }

                    if emergency_detected {
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
    }

    // Wait for the four data-producing threads.
    for handle in handles {
        handle.join().unwrap();
    }

    // ========================================================
    // THREAD 5: SIGNAL CONTROLLER
    // ========================================================

    let controller_state = Arc::clone(&state);

    let controller = thread::Builder::new()
        .name("signal-controller".to_string())
        .spawn(move || {
            println!("[Signal Controller] STARTED");

            let mut state =
                controller_state.lock().unwrap();

            println!(
                "[Signal Controller] Reading shared intersection state..."
            );

            println!(
                "[Signal Controller] North/South vehicles: {}",
                state.north_south_vehicles
            );

            println!(
                "[Signal Controller] East/West vehicles: {}",
                state.east_west_vehicles
            );

            println!(
                "[Signal Controller] Pedestrians waiting: {}",
                state.pedestrians_waiting
            );

            println!(
                "[Signal Controller] Emergency detected: {}",
                state.emergency_vehicle_detected
            );

            if state.emergency_vehicle_detected {
                state.signal_decision =
                    "Give priority to emergency vehicle".to_string();
            } else if state.north_south_vehicles
                > state.east_west_vehicles
            {
                state.signal_decision =
                    "Give North/South traffic longer green time"
                        .to_string();
            } else if state.east_west_vehicles
                > state.north_south_vehicles
            {
                state.signal_decision =
                    "Give East/West traffic longer green time"
                        .to_string();
            } else if state.pedestrians_waiting > 0 {
                state.signal_decision =
                    "Activate pedestrian crossing phase".to_string();
            } else {
                state.signal_decision =
                    "Use normal signal timing".to_string();
            }

            println!(
                "[Signal Controller] Work: decision = {}",
                state.signal_decision
            );

            println!("[Signal Controller] FINISHED");
        })
        .expect("Failed to create signal controller");

    controller.join().unwrap();

    println!();
    println!("Final shared intersection state:");

    let final_state =
        state.lock().unwrap();

    println!("{:#?}", *final_state);

    println!();
    println!("All five coordinated threads finished.");
}

// ============================================================
// PART 2A: UNSYNCHRONIZED TRAFFIC COUNTER
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
        "Each sensor reports {VEHICLES_PER_SENSOR} vehicle detections."
    );

    /*
        AtomicUsize makes each individual load and store atomic.

        However, the full sequence:

            load
            add 1
            store

        is NOT one atomic operation.

        Two threads can load the same old value and then both
        store the same incremented value.

        That creates a lost-update logical race.
    */

    let total_vehicles =
        Arc::new(AtomicUsize::new(0));

    let starting_barrier =
        Arc::new(Barrier::new(SENSOR_COUNT));

    let mut handles =
        Vec::new();

    for sensor_id in 1..=SENSOR_COUNT {
        let counter =
            Arc::clone(&total_vehicles);

        let barrier =
            Arc::clone(&starting_barrier);

        let name =
            format!("road-sensor-{sensor_id}");

        let handle =
            thread::Builder::new()
                .name(name.clone())
                .spawn(move || {
                    println!("[{name}] STARTED");

                    // All sensor threads begin together.
                    barrier.wait();

                    for detection in 0..VEHICLES_PER_SENSOR {
                        let old_value =
                            counter.load(Ordering::Relaxed);

                        // Encourage more thread interleaving.
                        if detection % 50 == 0 {
                            thread::yield_now();
                        }

                        counter.store(
                            old_value + 1,
                            Ordering::Relaxed,
                        );
                    }

                    println!(
                        "[{name}] Work: reported {VEHICLES_PER_SENSOR} vehicles"
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
            "Run the test again because thread scheduling is nondeterministic."
        );
    }
}

// ============================================================
// PART 2B: SYNCHRONIZED TRAFFIC COUNTER
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
        Arc allows all threads to share ownership.

        Mutex ensures that only one thread at a time
        can modify the shared counter.
    */

    let total_vehicles =
        Arc::new(Mutex::new(0usize));

    let starting_barrier =
        Arc::new(Barrier::new(SENSOR_COUNT));

    let mut handles =
        Vec::new();

    for sensor_id in 1..=SENSOR_COUNT {
        let counter =
            Arc::clone(&total_vehicles);

        let barrier =
            Arc::clone(&starting_barrier);

        let name =
            format!("protected-road-sensor-{sensor_id}");

        let handle =
            thread::Builder::new()
                .name(name.clone())
                .spawn(move || {
                    println!("[{name}] STARTED");

                    barrier.wait();

                    for _ in 0..VEHICLES_PER_SENSOR {
                        {
                            let mut vehicle_count =
                                counter.lock().unwrap();

                            *vehicle_count += 1;
                        }
                    }

                    println!(
                        "[{name}] Work: safely recorded {VEHICLES_PER_SENSOR} vehicles"
                    );

                    println!("[{name}] FINISHED");
                })
                .expect(
                    "Failed to create protected road sensor",
                );

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
    } else {
        println!();
        println!(
            "RESULT: Unexpected incorrect count."
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
// PART 3: LINUX THREAD SCHEDULING
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

    let starting_nice =
        current_nice();

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

    // ========================================================
    // ROUND 1: DEFAULT / NATURAL SCHEDULING
    // ========================================================

    println!();
    println!("==================================================");
    println!("ROUND 1: DEFAULT SCHEDULING");
    println!("==================================================");

    println!(
        "All three threads use the same default nice value."
    );

    run_scheduling_round(
        [
            starting_nice,
            starting_nice,
            starting_nice,
        ],
        selected_cpu,
    );

    // ========================================================
    // ROUND 2: MODIFIED NICE VALUES
    // ========================================================

    println!();
    println!("==================================================");
    println!("ROUND 2: MODIFIED NICE VALUES");
    println!("==================================================");

    println!(
        "The three threads now use different Linux nice values."
    );

    run_scheduling_round(
        [
            starting_nice,
            (starting_nice + 5).min(19),
            (starting_nice + 10).min(19),
        ],
        selected_cpu,
    );
}

fn run_scheduling_round(
    nice_values: [i32; 3],
    selected_cpu: Option<usize>,
) {
    let tasks = [
        "Emergency Traffic Analysis",
        "Congestion Analysis",
        "Historical Statistics",
    ];

    let barrier =
        Arc::new(Barrier::new(3));

    let mut handles =
        Vec::new();

    for index in 0..3 {
        let barrier =
            Arc::clone(&barrier);

        let cpu =
            selected_cpu;

        let task_name =
            tasks[index].to_string();

        let requested_nice =
            nice_values[index];

        let handle =
            thread::Builder::new()
                .name(task_name.clone())
                .spawn(move || {
                    println!();
                    println!(
                        "[{task_name}] STARTED"
                    );

                    println!(
                        "[{task_name}] Requested nice value: {requested_nice}"
                    );

                    // Pin all three workers to one CPU so that
                    // they compete directly for CPU time.
                    if let Some(cpu_number) = cpu {
                        if !pin_current_thread_to_cpu(
                            cpu_number,
                        ) {
                            println!(
                                "[{task_name}] Warning: CPU affinity failed."
                            );
                        }
                    }

                    // Change this thread's Linux nice value.
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

                    // Start the CPU-intensive work at
                    // approximately the same time.
                    barrier.wait();

                    let start =
                        Instant::now();

                    let run_time =
                        Duration::from_secs(3);

                    let mut calculations:
                        u64 = 0;

                    while start.elapsed() < run_time {
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
        |result| result.actual_nice
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
}

// ============================================================
// LINUX-SPECIFIC SCHEDULING FUNCTIONS
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