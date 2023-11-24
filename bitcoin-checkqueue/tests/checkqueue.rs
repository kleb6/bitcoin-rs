crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/checkqueue_tests.cpp]

#[cfg(test)]
#[fixture(TestingSetup)]
pub mod checkqueue_tests {

    pub const QUEUE_BATCH_SIZE:     u32 = 128;
    pub const SCRIPT_CHECK_THREADS: i32 = 3;

    pub struct FakeCheck { }

    impl FakeCheck {
        
        pub fn invoke(&self) -> bool {
            
            todo!();
            /*
                return true;
            */
        }
        
        pub fn swap(&mut self, x: &mut FakeCheck)  {
            
            todo!();
            /*
                }{
            */
        }
    }

    ///------------------------
    pub struct FakeCheckCheckCompletion {

    }

    pub mod fake_check_check_completion {

        lazy_static!{
            /*
            static std::atomic<size_t> n_calls;
            */
        }
    }

    impl FakeCheckCheckCompletion {

        pub fn invoke(&mut self) -> bool {
            
            todo!();
            /*
                n_calls.fetch_add(1, std::memory_order_relaxed);
                return true;
            */
        }
        
        pub fn swap(&mut self, x: &mut FakeCheckCheckCompletion)  { }
    }

    ///---------------------------
    pub struct FailingCheck {
        fails: bool,
    }

    impl FailingCheck {
        
        pub fn new(fails: bool) -> Self {
        
            todo!();
            /*
            : fails(_fails),

                }{
            */
        }
        
        fn default() -> Self {
            todo!();
            /*
            : fails(true),

                }{
            */
        }
        
        pub fn invoke(&self) -> bool {
            
            todo!();
            /*
                return !fails;
            */
        }
        
        pub fn swap(&mut self, x: &mut FailingCheck)  {
            
            todo!();
            /*
                std::swap(fails, x.fails);
            }{
            */
        }
    }

    ///-------------------------
    pub struct UniqueCheck {
        check_id: usize,
    }

    pub mod unique_check {
        lazy_static!{
            /*
            static Mutex m;
                static std::unordered_multiset<size_t> results GUARDED_BY(m);
            */
        }
    }

    impl UniqueCheck {
        
        pub fn new(check_id_in: usize) -> Self {
        
            todo!();
            /*
            : check_id(check_id_in),

            */
        }
        
        fn default() -> Self {
            todo!();
            /*
            : check_id(0),

            */
        }
        
        pub fn invoke(&mut self) -> bool {
            
            todo!();
            /*
                LOCK(m);
                results.insert(check_id);
                return true;
            */
        }
        
        pub fn swap(&mut self, x: &mut UniqueCheck)  {
            
            todo!();
            /*
                std::swap(x.check_id, check_id); 
            */
        }
    }

    ///-----------------------
    #[derive(Default)]
    pub struct MemoryCheck {

        b: bool, // default = { false }
    }

    pub mod memory_check {
        lazy_static!{
            /*
            static std::atomic<size_t> fake_allocated_memory;
            */
        }
    }

    impl Drop for MemoryCheck {
        fn drop(&mut self) {
            todo!();
            /*
                fake_allocated_memory.fetch_sub(b, std::memory_order_relaxed);
            */
        }
    }

    impl MemoryCheck {

        pub fn invoke(&self) -> bool {
            
            todo!();
            /*
                return true;
            */
        }
        
        pub fn new(x: &MemoryCheck) -> Self {
        
            todo!();
            /*


                // We have to do this to make sure that destructor calls are paired
                //
                // Really, copy constructor should be deletable, but CCheckQueue breaks
                // if it is deleted because of internal push_back.
                fake_allocated_memory.fetch_add(b, std::memory_order_relaxed);
            */
        }
        
        pub fn new(b: bool) -> Self {
        
            todo!();
            /*
            : b(b_),

                fake_allocated_memory.fetch_add(b, std::memory_order_relaxed);
            */
        }
        
        pub fn swap(&mut self, x: &mut MemoryCheck)  {
            
            todo!();
            /*
                std::swap(b, x.b); 
            */
        }
    }

    ///-----------------------
    pub struct FrozenCleanupCheck {

        /**
          | Freezing can't be the default initialized
          | behavior given how the queue swaps in
          | default initialized Checks.
          |
          */
        should_freeze: bool, // default = { false }
    }

    pub mod frozen_cleanup_check {
        lazy_static!{
            /*
            static std::atomic<uint64_t> nFrozen;
                static std::condition_variable cv;
                static std::mutex m;
            */
        }
    }

    impl Drop for FrozenCleanupCheck {
        fn drop(&mut self) {
            todo!();
            /*
                if (should_freeze) {
                    std::unique_lock<std::mutex> l(m);
                    nFrozen.store(1, std::memory_order_relaxed);
                    cv.notify_one();
                    cv.wait(l, []{ return nFrozen.load(std::memory_order_relaxed) == 0;});
                }
            */
        }
    }

    impl FrozenCleanupCheck {

        
        pub fn invoke(&self) -> bool {
            
            todo!();
            /*
                return true;
            */
        }
        
        pub fn swap(&mut self, x: &mut FrozenCleanupCheck)  {
            
            todo!();
            /*
                std::swap(should_freeze, x.should_freeze);}{
            */
        }
    }

    /* -------------- Static Allocations  -------------- */
    lazy_static!{
        /*
        std::mutex FrozenCleanupCheck::m{};
        std::atomic<uint64_t> FrozenCleanupCheck::nFrozen{0};
        std::condition_variable FrozenCleanupCheck::cv{};
        Mutex UniqueCheck::m;
        std::unordered_multiset<size_t> UniqueCheck::results;
        std::atomic<size_t> FakeCheckCheckCompletion::n_calls{0};
        std::atomic<size_t> MemoryCheck::fake_allocated_memory{0};
        */
    }


    /* ---------------- Queue Typedefs  ---------------- */
    pub type Correct_Queue       = CheckQueue<FakeCheckCheckCompletion>;
    pub type Standard_Queue      = CheckQueue<FakeCheck>;
    pub type Failing_Queue       = CheckQueue<FailingCheck>;
    pub type Unique_Queue        = CheckQueue<UniqueCheck>;
    pub type Memory_Queue        = CheckQueue<MemoryCheck>;
    pub type FrozenCleanup_Queue = CheckQueue<FrozenCleanupCheck>;

    /**
      | This test case checks that the CCheckQueue
      | works properly with each specified
      | size_t Checks pushed.
      |
      */
    pub fn correct_queue_range(range: Vec<usize>)  {
        
        todo!();
            /*
                auto small_queue = std::make_unique<Correct_Queue>(QUEUE_BATCH_SIZE);
            small_queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);
            // Make vChecks here to save on malloc (this test can be slow...)
            std::vector<FakeCheckCheckCompletion> vChecks;
            for (const size_t i : range) {
                size_t total = i;
                FakeCheckCheckCompletion::n_calls = 0;
                CCheckQueueControl<FakeCheckCheckCompletion> control(small_queue.get());
                while (total) {
                    vChecks.resize(std::min(total, (size_t) InsecureRandRange(10)));
                    total -= vChecks.size();
                    control.Add(vChecks);
                }
                BOOST_REQUIRE(control.Wait());
                if (FakeCheckCheckCompletion::n_calls != i) {
                    BOOST_REQUIRE_EQUAL(FakeCheckCheckCompletion::n_calls, i);
                }
            }
            small_queue->StopWorkerThreads();
            */
    }

    /**
      | Test that 0 checks is correct
      |
      */
    #[test] fn test_check_queue_correct_zero() {
        todo!();
        /*
        
            std::vector<size_t> range;
            range.push_back((size_t)0);
            Correct_Queue_range(range);

        */
    }

    /**
      | Test that 1 check is correct
      |
      */
    #[test] fn test_check_queue_correct_one() {
        todo!();
        /*
        
            std::vector<size_t> range;
            range.push_back((size_t)1);
            Correct_Queue_range(range);

        */
    }

    /**
      | Test that MAX check is correct
      |
      */
    #[test] fn test_check_queue_correct_max() {
        todo!();
        /*
        
            std::vector<size_t> range;
            range.push_back(100000);
            Correct_Queue_range(range);

        */
    }

    /**
      | Test that random numbers of checks are
      | correct
      |
      */
    #[test] fn test_check_queue_correct_random() {
        todo!();
        /*
        
            std::vector<size_t> range;
            range.reserve(100000/1000);
            for (size_t i = 2; i < 100000; i += std::max((size_t)1, (size_t)InsecureRandRange(std::min((size_t)1000, ((size_t)100000) - i))))
                range.push_back(i);
            Correct_Queue_range(range);

        */
    }

    /**
      | Test that failing checks are caught
      |
      */
    #[test] fn test_check_queue_catches_failure() {
        todo!();
        /*
        
            auto fail_queue = std::make_unique<Failing_Queue>(QUEUE_BATCH_SIZE);
            fail_queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);

            for (size_t i = 0; i < 1001; ++i) {
                CCheckQueueControl<FailingCheck> control(fail_queue.get());
                size_t remaining = i;
                while (remaining) {
                    size_t r = InsecureRandRange(10);

                    std::vector<FailingCheck> vChecks;
                    vChecks.reserve(r);
                    for (size_t k = 0; k < r && remaining; k++, remaining--)
                        vChecks.emplace_back(remaining == 1);
                    control.Add(vChecks);
                }
                bool success = control.Wait();
                if (i > 0) {
                    BOOST_REQUIRE(!success);
                } else if (i == 0) {
                    BOOST_REQUIRE(success);
                }
            }
            fail_queue->StopWorkerThreads();

        */
    }

    /**
      | Test that a block validation which fails
      | does not interfere with future blocks,
      | ie, the bad state is cleared.
      |
      */
    #[test] fn test_check_queue_recovers_from_failure() {
        todo!();
        /*
        
            auto fail_queue = std::make_unique<Failing_Queue>(QUEUE_BATCH_SIZE);
            fail_queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);

            for (auto times = 0; times < 10; ++times) {
                for (const bool end_fails : {true, false}) {
                    CCheckQueueControl<FailingCheck> control(fail_queue.get());
                    {
                        std::vector<FailingCheck> vChecks;
                        vChecks.resize(100, false);
                        vChecks[99] = end_fails;
                        control.Add(vChecks);
                    }
                    bool r =control.Wait();
                    BOOST_REQUIRE(r != end_fails);
                }
            }
            fail_queue->StopWorkerThreads();

        */
    }

    /**
      | Test that unique checks are actually all called
      | individually, rather than just one check being
      | called repeatedly. Test that checks are not
      | called more than once as well
      */
    #[test] fn test_check_queue_unique() {
        todo!();
        /*
        
            auto queue = std::make_unique<Unique_Queue>(QUEUE_BATCH_SIZE);
            queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);

            size_t COUNT = 100000;
            size_t total = COUNT;
            {
                CCheckQueueControl<UniqueCheck> control(queue.get());
                while (total) {
                    size_t r = InsecureRandRange(10);
                    std::vector<UniqueCheck> vChecks;
                    for (size_t k = 0; k < r && total; k++)
                        vChecks.emplace_back(--total);
                    control.Add(vChecks);
                }
            }
            {
                LOCK(UniqueCheck::m);
                bool r = true;
                BOOST_REQUIRE_EQUAL(UniqueCheck::results.size(), COUNT);
                for (size_t i = 0; i < COUNT; ++i) {
                    r = r && UniqueCheck::results.count(i) == 1;
                }
                BOOST_REQUIRE(r);
            }
            queue->StopWorkerThreads();

        */
    }

    /**
      | Test that blocks which might allocate lots of
      | memory free their memory aggressively.
      |
      | This test attempts to catch a pathological case
      | where by lazily freeing checks might mean
      | leaving a check un-swapped out, and decreasing
      | by 1 each time could leave the data hanging
      | across a sequence of blocks.
      */
    #[test] fn test_check_queue_memory() {
        todo!();
        /*
        
            auto queue = std::make_unique<Memory_Queue>(QUEUE_BATCH_SIZE);
            queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);
            for (size_t i = 0; i < 1000; ++i) {
                size_t total = i;
                {
                    CCheckQueueControl<MemoryCheck> control(queue.get());
                    while (total) {
                        size_t r = InsecureRandRange(10);
                        std::vector<MemoryCheck> vChecks;
                        for (size_t k = 0; k < r && total; k++) {
                            total--;
                            // Each iteration leaves data at the front, back, and middle
                            // to catch any sort of deallocation failure
                            vChecks.emplace_back(total == 0 || total == i || total == i/2);
                        }
                        control.Add(vChecks);
                    }
                }
                BOOST_REQUIRE_EQUAL(MemoryCheck::fake_allocated_memory, 0U);
            }
            queue->StopWorkerThreads();

        */
    }

    /**
      | Test that a new verification cannot
      | occur until all checks have been destructed
      |
      */
    #[test] fn test_check_queue_frozen_cleanup() {
        todo!();
        /*
        
            auto queue = std::make_unique<FrozenCleanup_Queue>(QUEUE_BATCH_SIZE);
            bool fails = false;
            queue->StartWorkerThreads(SCRIPT_CHECK_THREADS);
            std::thread t0([&]() {
                CCheckQueueControl<FrozenCleanupCheck> control(queue.get());
                std::vector<FrozenCleanupCheck> vChecks(1);
                // Freezing can't be the default initialized behavior given how the queue
                // swaps in default initialized Checks (otherwise freezing destructor
                // would get called twice).
                vChecks[0].should_freeze = true;
                control.Add(vChecks);
                bool waitResult = control.Wait(); // Hangs here
                assert(waitResult);
            });
            {
                std::unique_lock<std::mutex> l(FrozenCleanupCheck::m);
                // Wait until the queue has finished all jobs and frozen
                FrozenCleanupCheck::cv.wait(l, [](){return FrozenCleanupCheck::nFrozen == 1;});
            }
            // Try to get control of the queue a bunch of times
            for (auto x = 0; x < 100 && !fails; ++x) {
                fails = queue->m_control_mutex.try_lock();
            }
            {
                // Unfreeze (we need lock n case of spurious wakeup)
                std::unique_lock<std::mutex> l(FrozenCleanupCheck::m);
                FrozenCleanupCheck::nFrozen = 0;
            }
            // Awaken frozen destructor
            FrozenCleanupCheck::cv.notify_one();
            // Wait for control to finish
            t0.join();
            BOOST_REQUIRE(!fails);
            queue->StopWorkerThreads();

        */
    }

    /**
      | Test that CCheckQueueControl is threadsafe
      |
      */
    #[test] fn test_check_queue_control_locks() {
        todo!();
        /*
        
            auto queue = std::make_unique<Standard_Queue>(QUEUE_BATCH_SIZE);
            {
                std::vector<std::thread> tg;
                std::atomic<int> nThreads {0};
                std::atomic<int> fails {0};
                for (size_t i = 0; i < 3; ++i) {
                    tg.emplace_back(
                            [&]{
                            CCheckQueueControl<FakeCheck> control(queue.get());
                            // While sleeping, no other thread should execute to this point
                            auto observed = ++nThreads;
                            UninterruptibleSleep(std::chrono::milliseconds{10});
                            fails += observed  != nThreads;
                            });
                }
                for (auto& thread: tg) {
                    if (thread.joinable()) thread.join();
                }
                BOOST_REQUIRE_EQUAL(fails, 0);
            }
            {
                std::vector<std::thread> tg;
                std::mutex m;
                std::condition_variable cv;
                bool has_lock{false};
                bool has_tried{false};
                bool done{false};
                bool done_ack{false};
                {
                    std::unique_lock<std::mutex> l(m);
                    tg.emplace_back([&]{
                            CCheckQueueControl<FakeCheck> control(queue.get());
                            std::unique_lock<std::mutex> ll(m);
                            has_lock = true;
                            cv.notify_one();
                            cv.wait(ll, [&]{return has_tried;});
                            done = true;
                            cv.notify_one();
                            // Wait until the done is acknowledged
                            //
                            cv.wait(ll, [&]{return done_ack;});
                            });
                    // Wait for thread to get the lock
                    cv.wait(l, [&](){return has_lock;});
                    bool fails = false;
                    for (auto x = 0; x < 100 && !fails; ++x) {
                        fails = queue->m_control_mutex.try_lock();
                    }
                    has_tried = true;
                    cv.notify_one();
                    cv.wait(l, [&](){return done;});
                    // Acknowledge the done
                    done_ack = true;
                    cv.notify_one();
                    BOOST_REQUIRE(!fails);
                }
                for (auto& thread: tg) {
                    if (thread.joinable()) thread.join();
                }
            }

        */
    }
}