use libc::c_uint;

bitflags! {
    #[doc="Environment Options"]
    #[deriving(Show)]
    flags EnvironmentFlags: c_uint {

        #[doc="Use a fixed address for the mmap region. This flag must be specified"]
        #[doc="when creating the environment, and is stored persistently in the environment."]
        #[doc="If successful, the memory map will always reside at the same virtual address"]
        #[doc="and pointers used to reference data items in the database will be constant"]
        #[doc="across multiple invocations. This option may not always work, depending on"]
        #[doc="how the operating system has allocated memory to shared libraries and other uses."]
        #[doc="The feature is highly experimental."]
        const MDB_FIXEDMAP = 0x01,

        #[doc="By default, LMDB creates its environment in a directory whose"]
        #[doc="pathname is given in `path`, and creates its data and lock files"]
        #[doc="under that directory. With this option, `path` is used as-is for"]
        #[doc="the database main data file. The database lock file is the `path`"]
        #[doc="with `-lock` appended."]
        const MDB_NOSUBDIR = 0x4000,

        #[doc="Use a writeable memory map unless `MDB_RDONLY` is set. This is faster and uses"]
        #[doc="fewer mallocs, but loses protection from application bugs like wild pointer writes"]
        #[doc="and other bad updates into the database. Incompatible with nested transactions."]
        #[doc="Processes with and without `MDB_WRITEMAP` on the same environment do not cooperate"]
        #[doc="well."]
        const MDB_WRITEMAP = 0x80000,

        #[doc="Open the environment in read-only mode. No write operations will be allowed."]
        #[doc="When opening an environment, LMDB will still modify the lock file - except on"]
        #[doc="read-only filesystems, where LMDB does not use locks."]
        const MDB_RDONLY = 0x20000,

        #[doc="Flush system buffers to disk only once per transaction, omit the"]
        #[doc="metadata flush. Defer that until the system flushes files to disk,"]
        #[doc="or next non-`MDB_RDONLY` commit or #mdb_env_sync(). This optimization"]
        #[doc="maintains database integrity, but a system crash may undo the last"]
        #[doc="committed transaction. I.e. it preserves the ACI (atomicity,"]
        #[doc="consistency, isolation) but not D (durability) database property."]
        #[doc="\n\nThis flag may be changed at any time using `Environment::set_flags`."]
        const MDB_NOMETASYNC = 0x40000,

        #[doc="Don't flush system buffers to disk when committing a transaction."]
        #[doc="This optimization means a system crash can corrupt the database or"]
        #[doc="lose the last transactions if buffers are not yet flushed to disk."]
        #[doc="The risk is governed by how often the system flushes dirty buffers"]
        #[doc="to disk and how often `Environment::sync` is called.  However, if the"]
        #[doc="filesystem preserves write order and the `MDB_WRITEMAP` flag is not"]
        #[doc="used, transactions exhibit ACI (atomicity, consistency, isolation)"]
        #[doc="properties and only lose D (durability).  I.e. database integrity"]
        #[doc="is maintained, but a system crash may undo the final transactions."]
        #[doc="Note that (`MDB_NOSYNC | MDB_WRITEMAP`) leaves the system with no"]
        #[doc="hint for when to write transactions to disk, unless `Environment::sync`"]
        #[doc="is called. (`MDB_MAPASYNC | MDB_WRITEMAP`) may be preferable."]
        #[doc="\n\nThis flag may be changed at any time using `Environment::set_flags`."]
        const MDB_NOSYNC = 0x10000,

        #[doc="When using `MDB_WRITEMAP`, use asynchronous flushes to disk."]
        #[doc="As with `MDB_NOSYNC`, a system crash can then corrupt the"]
        #[doc="database or lose the last transactions. Calling `Environment::sync`"]
        #[doc="ensures on-disk database integrity until next commit."]
        #[doc="\n\nThis flag may be changed at any time using `Environment::set_flags`."]
        const MDB_MAPASYNC = 0x100000,

        #[doc="Don't use Thread-Local Storage. Tie reader locktable slots to"]
        #[doc="transaction objects instead of to threads. I.e. `RoTransaction::reset` keeps"]
        #[doc="the slot reseved for the transaction object. A thread may use parallel"]
        #[doc="read-only transactions. A read-only transaction may span threads if"]
        #[doc="the user synchronizes its use. Applications that multiplex many"]
        #[doc="user threads over individual OS threads need this option. Such an"]
        #[doc="application must also serialize the write transactions in an OS"]
        #[doc="thread, since LMDB's write locking is unaware of the user threads."]
        const MDB_NOTLS = 0x200000,

        #[doc="Don't do any locking. If concurrent access is anticipated, the"]
        #[doc="caller must manage all concurrency themself. For proper operation"]
        #[doc="the caller must enforce single-writer semantics, and must ensure"]
        #[doc="that no readers are using old transactions while a writer is"]
        #[doc="active. The simplest approach is to use an exclusive lock so that"]
        #[doc="no readers may be active at all when a writer begins."]
        const MDB_NOLOCK = 0x400000,

        #[doc="Turn off readahead. Most operating systems perform readahead on"]
        #[doc="read requests by default. This option turns it off if the OS"]
        #[doc="supports it. Turning it off may help random read performance"]
        #[doc="when the DB is larger than RAM and system RAM is full."]
        #[doc="The option is not implemented on Windows."]
        const MDB_NORDAHEAD = 0x800000,

        #[doc="Don't initialize malloc'd memory before writing to unused spaces"]
        #[doc="in the data file. By default, memory for pages written to the data"]
        #[doc="file is obtained using malloc. While these pages may be reused in"]
        #[doc="subsequent transactions, freshly malloc'd pages will be initialized"]
        #[doc="to zeroes before use. This avoids persisting leftover data from other"]
        #[doc="code (that used the heap and subsequently freed the memory) into the"]
        #[doc="data file. Note that many other system libraries may allocate"]
        #[doc="and free memory from the heap for arbitrary uses. E.g., stdio may"]
        #[doc="use the heap for file I/O buffers. This initialization step has a"]
        #[doc="modest performance cost so some applications may want to disable"]
        #[doc="it using this flag. This option can be a problem for applications"]
        #[doc="which handle sensitive data like passwords, and it makes memory"]
        #[doc="checkers like Valgrind noisy. This flag is not needed with `MDB_WRITEMAP`,"]
        #[doc="which writes directly to the mmap instead of using malloc for pages. The"]
        #[doc="initialization is also skipped if `MDB_RESERVE` is used; the"]
        #[doc="caller is expected to overwrite all of the memory that was"]
        #[doc="reserved in that case."]
        #[doc="\n\nThis flag may be changed at any time using `Environment::set_flags`."]
        const MDB_NOMEMINIT = 0x1000000,
    }
}

bitflags! {
    #[doc="Database Options"]
    #[deriving(Show)]
    flags DatabaseFlags: c_uint {

            #[doc="Keys are strings to be compared in reverse order, from the end"]
            #[doc="of the strings to the beginning. By default, Keys are treated as strings and"]
            #[doc="compared from beginning to end."]
            const MDB_REVERSEKEY = 0x02,

            #[doc="Duplicate keys may be used in the database. (Or, from another perspective,"]
            #[doc="keys may have multiple data items, stored in sorted order.) By default"]
            #[doc="keys must be unique and may have only a single data item."]
            const MDB_DUPSORT = 0x04,

            #[doc="Keys are binary integers in native byte order. Setting this option"]
            #[doc="requires all keys to be the same size, typically sizeof(int)"]
            #[doc="or sizeof(size_t)."]
            const MDB_INTEGERKEY = 0x08,

            #[doc="This flag may only be used in combination with `MDB_DUPSORT`. This option"]
            #[doc="tells the library that the data items for this database are all the same"]
            #[doc="size, which allows further optimizations in storage and retrieval. When"]
            #[doc="all data items are the same size, the `MDB_GET_MULTIPLE` and `MDB_NEXT_MULTIPLE`"]
            #[doc="cursor operations may be used to retrieve multiple items at once."]
            const MDB_DUPFIXED = 0x10,

            #[doc="This option specifies that duplicate data items are also integers, and"]
            #[doc="should be sorted as such."]
            const MDB_INTEGERDUP = 0x20,

            #[doc="This option specifies that duplicate data items should be compared as"]
            #[doc="strings in reverse order."]
            const MDB_REVERSEDUP = 0x40,
    }
}

bitflags! {
    #[doc="Write Options"]
    #[deriving(Show)]
    flags WriteFlags: c_uint {

        #[doc="Insert the new item only if the key does not already appear in the database."]
        #[doc="The function will return `LmdbError::KeyExist` if the key already appears in the"]
        #[doc="database, even if the database supports duplicates (`MDB_DUPSORT`)."]
        const MDB_NOOVERWRITE = 0x10,

        #[doc="Insert the new item only if it does not already appear in the database."]
        #[doc="This flag may only be specified if the database was opened with `MDB_DUPSORT`."]
        #[doc="The function will return `LmdbError::KeyExist` if the item already appears in the"]
        #[doc="database."]
        const MDB_NODUPDATA = 0x20,

        #[doc="For `Cursor::put`. Replace the item at the current cursor position."]
        #[doc="The key parameter must match the current position. If using"]
        #[doc="sorted duplicates (`MDB_DUPSORT`) the data item must still sort into the"]
        #[doc="same position. This is intended to be used when the new data is the same"]
        #[doc="size as the old. Otherwise it will simply perform a delete of the old"]
        #[doc="record followed by an insert."]
        const MDB_CURRENT = 0x40,

        #[doc="Append the given item to the end of the database. No key comparisons are performed."]
        #[doc="This option allows fast bulk loading when keys are already known to be in the"]
        #[doc="correct order. Loading unsorted keys with this flag will cause data corruption."]
        const MDB_APPEND = 0x20000,

        #[doc="Same as `MDB_APPEND`, but for sorted dup data."]
        const MDB_APPENDDUP = 0x40000,
    }
}
