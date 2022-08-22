initSidebarItems({"fn":[["exec_new_task","通过 exec 系统调用，直接切换到新的用户进程"],["exit_current_task","终止当前用户程序，回到 idle 状态"],["get_current_task","获取当前核正在运行的进程的TCB。 如果当前核没有任务，则返回 None"],["handle_signals","处理当前线程的信号"],["handle_user_page_fault","处理用户程序的缺页异常"],["handle_zombie_task","处理退出的任务： 将它的子进程全部交给初始进程 ORIGIN_USER_PROC，然后标记当前进程的状态为 Zombie。 这里会需要获取当前核正在运行的用户程序、ORIGIN_USER_PROC、所有子进程的锁。"],["run_tasks","开始执行用户程序"],["signal_return","从信号处理中返回。 为了适配 syscall，返回原来的用户上下文中的 a0 的值"],["suspend_current_task","暂停当前用户程序，回到 idle 状态"],["timer_kernel_to_user","从内核态进入用户态时统计时间"],["timer_user_to_kernel","从用户态进入内核态时统计时间"]],"struct":[["CPU_CONTEXTS","所有 CPU 的上下文信息"],["CpuLocal","每个核当前正在运行的任务及上下文信息。 注意，如果一个核没有运行在任何任务上，那么它会回到 idle_task_cx 的上下文，而这里的栈就是启动时的栈。 启动时的栈空间在初始化内核 MemorySet 与页表时有留出 shadow page，也即如果在核空闲时不断嵌套异常中断导致溢出， 会在 trap 中进入 StorePageFault，然后panic终止系统"]]});