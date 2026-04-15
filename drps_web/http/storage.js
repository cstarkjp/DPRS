/**
 * This contains Directory, LocalStorage
 */
/* History
 *
 * 12 April:
 *  Converted to TypeScript (temporarily removed DbStorage)
 *  Renamed FileSet to LocalStorage
 *
 * 31 March: Directory methods take files in root, suffix rather than the other ways round
 *
 */
/**
 * A directory that contains sets of files identified by the 'root' with specific 'suffixes'
 *
 * The aim is to provide a simple means to list files of a specific 'suffix'
 */
export class Directory {
    /**
     * Create a new Directory with no contents
     */
    constructor() {
        this.files = new Map();
    }
    /**
     * Split the filename into a root and suffix
     */
    static split_filename(filename) {
        const suffix = filename.split(".").pop();
        if (suffix) {
            const root = filename.slice(0, -suffix.length - 1);
            return [root, suffix];
        }
        else {
            return null;
        }
    }
    /**
     * Clear the contents
     */
    clear() {
        this.files.clear();
    }
    /**
     * Determine if a file with a given suffix is in the directory
     *
     * @param root The root (basename) of the file
     * @param suffix The suffix (type) of the file
     * @return True if the file 'root.suffix' is in the directory
     */
    contains_file(root, suffix) {
        if (!this.files.has(suffix)) {
            return false;
        }
        return this.files.get(suffix).has(root);
    }
    /**
     * Add file to the directory; if it already exists, then keep it
     *
     * @param root The root (basename) of the file
     * @param suffix The suffix (type) of the file
     */
    add_file(root, suffix) {
        if (!this.files.has(suffix)) {
            this.files.set(suffix, new Set());
        }
        this.files.get(suffix).add(root);
    }
    /**
     * Remove a file from the directory; if it does not exist, then do nothing
     *
     * @param root The root (basename) of the file
     * @param suffix The suffix (type) of the file
     */
    delete_file(root, suffix) {
        if (!this.files.has(suffix)) {
            return;
        }
        this.files.get(suffix).delete(root);
        if (this.files.get(suffix).size == 0) {
            this.files.delete(suffix);
        }
    }
    /**
     * Retrieve all of the files with a particular suffix in this Directory
     *
     * @param suffix The suffix (type) of the file
     * @returns Set Iterator of all the root names of the files with the given suffix in the Directory
     */
    files_of_type(suffix) {
        const file_set = this.files.get(suffix);
        if (!file_set) {
            return null;
        }
        return file_set.keys();
    }
}
/**
 * A class that manages local storage using a 'prefix' into the actual storage (to permit more than one such class with an 'application')
 *
 */
export class LocalStorage {
    /**
     * Construct a new LocalStorage for a given prefix, and retrieve the directory contents
     *
     */
    constructor(storage, prefix) {
        this.storage = storage;
        this.prefix = prefix;
        this.directory = new Directory();
        this.load_dir();
    }
    /**
     * Load the directory from the storage
     *
     */
    load_dir() {
        this.directory.clear();
        const n = this.storage.length;
        const pl = this.prefix.length;
        for (let i = 0; i < n; i++) {
            let k = this.storage.key(i);
            if (k.startsWith(this.prefix)) {
                const f = k.slice(pl);
                const s_r = Directory.split_filename(f);
                if (s_r) {
                    this.directory.add_file(s_r[0], s_r[1]);
                }
            }
        }
    }
    /**
     * Load a file from Storage immediately
     *
     * This does not check to see if it is in the directory - it goes straight to the storage
     *
     */
    load_file(root, suffix) {
        let f = this.prefix + root + "." + suffix;
        return this.storage.getItem(f);
    }
    /**
     * Save a file to Storage
     *
     * This will add the file to the directory as well as storing it
     *
     */
    save_file(root, suffix, data) {
        let f = this.prefix + root + "." + suffix;
        this.storage.setItem(f, data);
        this.directory.add_file(root, suffix);
    }
    /**
     * Delete a file from Storage
     *
     * This will remove the file from the directory as well as deleting it
     *
     */
    delete_file(root, suffix) {
        let f = this.prefix + root + "." + suffix;
        this.storage.removeItem(f);
        this.directory.delete_file(root, suffix);
    }
    /**
     * Request to load a file from Storage, and invoke callback when it completes
     *
     * This does not check to see if it is in the directory - it goes straight to the storage
     *
     */
    request_load_file(filename, suffix, user_callback) {
        const data = this.load_file(filename, suffix);
        user_callback(data);
    }
    /**
     * Request to save a file to Storage, and invoke callback when it completes (with an indication of success or failure)
     *
     * This will add the file to the directory as well as storing it
     *
     */
    request_save_file(filename, suffix, data, user_callback) {
        this.save_file(filename, suffix, data);
        user_callback(true);
    }
    /**
     * Return the directory contents
     *
     */
    dir() {
        return this.directory;
    }
}
