initSidebarItems({"fn":[["allocator_init","初始化堆分配器、页帧分配器和 TID 分配器。需由其中一个核调用且仅调用一次"]],"mod":[["fd","文件描述符分配器"],["frame","页帧分配器"],["heap","堆分配器"],["tid","TID 分配器 最大支持 4096 个线程id。如需要更多，修改下面的 TidAllocatorImpl 即可 实际上u740板子的内存16G用不完bit map，但再小一点的实现只有4G空间"]]});