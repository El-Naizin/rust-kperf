use std::time::Instant;
use kperf_rs;
use kperf_rs::event::Event;
use kperf_rs::check_kpc_permission;
use kperf_rs::PerfCounterBuilder;
use libc::size_t;
use kperf_rs::kperf::ticks_to_nanoseconds;

const KPC_MAX_COUNTERS: size_t = 32;

fn main() {
    // Check permission
    check_kpc_permission().expect("KPC Permission denied");


    let iterations = 500000;
    let mut perf_counter = PerfCounterBuilder::new()
        .track_event(Event::Cycles)
        .build_counter()
        .unwrap();
    let mut perf_counter_2 = PerfCounterBuilder::new()
        .track_event(Event::Cycles)
        .build_counter()
        .unwrap();
    perf_counter_2
        .start()
        .expect("Failed to start thread counters");
    let now = Instant::now();
    perf_counter
        .start()
        .expect("Failed to start thread counters");

    for i in 0..iterations {
        let a = 4 + i % 5;
        let b = a * 3;
        let c = 4;
        if a > 3{
            let d = 1;
        } else {
            let d = 2;
        }
        format!("{a}, {b}, {c}");
    }
    perf_counter.stop().expect("Failed to stop thread counters");
    let elapsed_time = now.elapsed();
    perf_counter_2
        .stop()
        .expect("Failed to start thread counters");
    let counter_result = perf_counter.read();
    let counter_result_2 = perf_counter_2.read();
    println!(
        "perf1: Cycles: {}\nCycles per iteration: {}",
        counter_result,
        counter_result as f64 / iterations as f64
    );

    println!("\
    Cycles time           :\t{}\n\
    Cycles time (secs)    :\t{}\n\
    time crate time       :\t{}\n\
    time crate time (secs):\t{}\n\
    Diff                  :\t{}\
    ",
             ticks_to_nanoseconds(counter_result),
             (ticks_to_nanoseconds(counter_result)) as f64 / 1_000_000_000.,
             elapsed_time.as_nanos(),
             elapsed_time.as_secs_f64(),
             (ticks_to_nanoseconds(counter_result)) as i64 - elapsed_time.as_nanos() as i64
    );

    println!(
        "perf2: Cycles: {}\nCycles per iteration: {}",
        counter_result_2,
        counter_result_2 as f64 / iterations as f64
    );
    println!(
        "Difference perf1 and 2: {}",
        counter_result_2 - counter_result
    );
}

// fn main() {
//     // Check permission
//     check_kpc_permission().expect("KPC Permission denied");
//
//     let mut db = KProbesDatabase::load_database().expect("Couldn't load KProbes database");
//     // Load pmc database
//
//     // TODO: Make those functions of KProbesDatabase
//     // let db_name = CStr::from_ptr((*db).name);
//     // let db_marketing_name = CStr::from_ptr((*db).marketing_name);
//     // println!("loaded db: {:?} ({:?})", db_name, db_marketing_name);
//
//     let config = KProbesConfig::from_database(&mut db).expect("Failed to create kpep config");
//
//     unsafe {
//         let mut res = kpep_config_force_counters(config.config);
//         if res != 0 {
//             println!(
//                 "Failed to force counters, error: {}",
//                 res
//             )
//         }
//
//         let mut ev_branches = get_event(Event::Branches, &mut *(db.database)).unwrap();
//         kpep_config_add_event(config.config, &mut ev_branches, 0, null_mut());
//         let mut ev_cycles = get_event(Event::Cycles, &mut *(db.database)).unwrap();
//         kpep_config_add_event(config.config, &mut ev_cycles, 0, null_mut());
//
//         let mut classes = 0;
//         let mut reg_count = 0;
//         let res = kpep_config_kpc_classes(config.config, &mut classes);
//         if res != 0 {
//             println!(
//                 "Failed to get kpc classes, error: {}",
//                 res
//             )
//         }
//         println!(
//             "kpc classes: {}",
//             classes
//         );
//         let res = kpep_config_kpc_count(config.config, &mut reg_count);
//         if res != 0 {
//             println!(
//                 "Failed to get kpc count, error: {}",
//                 res
//             )
//         }
//         println!(
//             "kpc count: {}",
//             reg_count
//         );
//         let mut counter_map: [size_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
//         println!(
//             "{}",
//             size_of::<[size_t; KPC_MAX_COUNTERS]>()
//         );
//         let res = kpep_config_kpc_map(config.config, counter_map.as_mut_ptr(), size_of::<[size_t; KPC_MAX_COUNTERS]>());
//         if res != 0 {
//             println!(
//                 "Failed to initialize kpc map, error: {}",
//                 res
//             )
//         }
//         println!(
//             "Counter map: {:?}",
//             counter_map,
//         );
//
//         let mut regs: [kpc_config_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
//         let res = kpep_config_kpc(config.config, regs.as_mut_ptr(), size_of::<[kpc_config_t; KPC_MAX_COUNTERS]>());
//         if res != 0 {
//             println!(
//                 "Failed to configure kpc, error: {}",
//                 res
//             );
//         }
//
//         kpc_force_all_ctrs_set(1); // Set config to kernel
//         if (classes & KPC_CLASS_CONFIGURABLE_MASK) != 0 && reg_count != 0 {
//             let res = kpc_set_config(classes, regs.as_mut_ptr());
//             if res != 0 {
//                 println!(
//                     "Failed to set kpc config, error: {}",
//                     res
//                 );
//             }
//         }
//         println!(
//             "regs = {:?}",
//             regs
//         );
//
//         // let res = kpc_set_counting(classes);
//         // if res != 0 {
//         //     println!(
//         //         "Failed to start kpc counting, error: {}",
//         //         res
//         //     );
//         // }
//
//         let res = kpc_set_thread_counting(classes);
//         if res != 0 {
//             println!(
//                 "Failed to start kpc thread counting, error: {}",
//                 res
//             )
//         }
//         println!("Should have worked");
//
//         // Get counters
//         let mut counters_0_start = [0 as c_ulonglong; KPC_MAX_COUNTERS];
//         let res = kpc_get_thread_counters(0, KPC_MAX_COUNTERS as c_uint, counters_0_start.as_mut_ptr());
//         if res != 0 {
//             println!(
//                 "Failed to get thread counters : {}",
//                 res
//             );
//         }
//
//         // Get counters
//         let mut counters_0_end = [0 as c_ulonglong; KPC_MAX_COUNTERS];
//         let res = kpc_get_thread_counters(0, KPC_MAX_COUNTERS as c_uint, counters_0_end.as_mut_ptr());
//         if res != 0 {
//             println!(
//                 "Failed to get thread counters : {}",
//                 res
//             );
//         }
//         println!("Got counters start and end");
//
//         let mut counters_0_diff = counters_0_end;
//         println!("Initialized counters_0_diff");
//         println!("diff: {:?}\nstart: {:?}\nend: {:?}", counters_0_diff, counters_0_start, counters_0_end);
//
//         counters_0_diff[counter_map[0]] -= counters_0_start[counter_map[0]];
//
//         println!("Cycles: {:?}", counters_0_diff[counter_map[0]]);
//         println!("Branches: {:?}", counters_0_diff[counter_map[3]]); // Not working yet
//         println!("Missed Branches: {:?}", counters_0_diff[counter_map[2]]); // Not working yet
//         println!("Instructions: {:?}", counters_0_diff[counter_map[1]]); // Not working yet
//
//     }
//     println!("Hello, world!");
// }
