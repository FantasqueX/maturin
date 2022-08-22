initSidebarItems({"fn":[["add_link","添加硬链接"],["get_link_count","获取硬链接数。"],["mount_fat_fs","挂载一个fatfs类型的设备"],["parse_file_name","将用户提供的路径和文件转换成实际的路径和文件"],["read_link","检查文件名对应的链接 如果在 map 中找不到对应链接，则返回 None"],["try_add_link","尝试添加一个硬链接。左边是实际路径和文件，右边是作为链接的路径和文件"],["try_add_rev_link","尝试添加一个硬链接。左边是作为链接的路径和文件，右边是实际路径和文件"],["try_remove_link","尝试删除一个硬链接。 如果链接数为0，则删除该文件。"],["umount_fat_fs",""]],"static":[["LINK_COUNT_MAP","实际文件(而不是用户文件)到链接数的映射"],["LINK_PATH_MAP","用户看到的文件到实际文件的映射"],["MOUNTED","已挂载的文件系统(设备)。 注意启动时的文件系统不在这个 vec 里，它在 mod.rs 里。"]],"struct":[["FileDisc","同时保存文件路径和文件名，作为链接表的 K/V"],["MountedFs","挂载的文件系统。 目前“挂载“的语义是，把一个文件当作文件系统读写 TODO: 把 mod.rs 中文件系统的操作全部封装为 struct，然后挂载时用文件实例化它"]]});