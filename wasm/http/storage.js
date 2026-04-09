/**
 * A directory of files, that is stored sepearately in some medium
 *
 * @param input Description of parameter.
 * @return Description of return value.
 * @throws Exception Description of exception.
 */

/* History
 *
 * 31 March: Directory methods take files in root, suffix rather than the other ways round
 *
 */
export class Directory {
  /**
   * Create a new Directory with no contents
   */
  constructor() {
    this.files = {};
  }

  clear() {
    this.files = {};
  }
  /**
   * Determine if a file with a given suffix is in the directory
   *
   * @param root The root (basename) of the file
   * @param suffix The suffix (type) of the file
   * @return True if the file 'root.suffix' is in the directory
   */
  contains_file(root, suffix) {
    if (!this.files[suffix]) {
      return false;
    }
    if (!this.files[suffix][root]) {
      return false;
    }
    return true;
  }

  /**
   * Add file to the directory; if it already exists, then keep it
   *
   * @param root The root (basename) of the file
   * @param suffix The suffix (type) of the file
   */
  add_file(root, suffix) {
    if (!this.files[suffix]) {
      this.files[suffix] = {};
    }
    this.files[suffix][root] = true;
  }

  /**
   * Remove a file from the directory; if it does not exist, then do nothing
   *
   * @param root The root (basename) of the file
   * @param suffix The suffix (type) of the file
   */
  delete_file(root, suffix) {
    if (!this.files[suffix]) {
      return;
    }

    if (!this.files[suffix][root]) {
      return;
    }

    delete this.files[suffix][root];
    if (this.files[suffix].length == 0) {
      delete this.files[suffix];
    }
  }

  /**
   * Retrieve all of the files with a particular suffix in this Directory
   *
   * @param suffix The suffix (type) of the file
   * @returns Array with all the root names of the files with the given suffix in the Directory
   */
  files_of_type(suffix) {
    if (!this.files[suffix]) {
      return [];
    }
    return Object.keys(this.files[suffix]);
  }
}

export class DBStorage {
  /**
   * Construct a database (a browser IndexedDB) for storing binary (and other) files
   *
   * Invokes the callback when the DBStorage is ready
   *
   * @param event
   */
  constructor(db_name, init_callback) {
    const DBOpenRequest = window.indexedDB.open(db_name, 1);
    this.db = null;
    this.db_init_callback = init_callback;
    this.db_name = db_name;
    this.directory = new Directory();
    DBOpenRequest.onerror = (event) => {
      this.db_open_error(event);
    };
    DBOpenRequest.oncomplete = (event) => {
      this.db_open_complete(event);
    };
    DBOpenRequest.onupgradeneeded = (event) => {
      this.db_upgrade(event);
    };
    DBOpenRequest.onsuccess = (event) => {
      this.db_open_success(event);
    };
  }

  /**
   * Internal method invoked when the database fails to open
   *
   * Invokes the db_callback with 'false'
   *
   * @param event
   */
  db_open_error(event) {
    console.error(`Error loading Fonts database: ${event}`);
    this.db_init_callback(false);
  }

  /**
   * Internal method invoked when the database open completes
   *
   * This occurs ??
   *
   * Invokes the db_callback with 'false'
   *
   * @param event
   */
  db_open_complete(event) {
    console.info("Completed DbOpen.");
  }

  /**
   * Internal method invoked when the database open requires an update to the database (including initialization from nothing)
   *
   * After this a 'success' event should occur
   *
   * @param event
   */
  db_upgrade(event) {
    // This occurs if the database does not exist or is at a lower version number
    //
    // Once this has done things the database should later open successfully
    console.log("Database needs to be set up");
    this.db = event.target.result;
    if (!this.db.objectStoreNames.contains("storage")) {
      console.log("Creating 'storage' in Fonts");
      this.db.createObjectStore("storage", { keyPath: "filename" });
    }
  }

  /**
   * Internal method invoked when the database open succeeds
   *
   * After this occurs the 'db' property will be set up.
   *
   * Invokes the db_callback with 'true'
   *
   * @param event
   */
  db_open_success(event) {
    console.log("Database opened ok");
    this.db = event.target.result;
    this.db_init_callback(true);
  }

  db_request_complete(success, callback, user_callback, data) {
    callback(user_callback, success, data);
  }

  db_read_request(request_fn, callback, user_callback) {
    const transaction = this.db.transaction("storage", "readonly");
    const storage = transaction.objectStore("storage");
    const request = request_fn(storage);

    request.onsuccess = (result) => {
      const db_result = result.target.result;
      if (db_result) {
        this.db_request_complete(true, callback, user_callback, db_result);
      } else {
        this.db_request_complete(false, callback, user_callback, null);
      }
    };
    request.onerror = (error) => {
      this.db_request_complete(false, callback, user_callback, error);
    };
  }

  db_readwrite_request(request_fn, callback, user_callback) {
    const transaction = this.db.transaction("storage", "readwrite");
    const storage = transaction.objectStore("storage");
    const request = request_fn(storage);

    request.onsuccess = (result) => {
      this.db_request_complete(true, callback, user_callback, result);
    };
    request.onerror = (error) => {
      this.db_request_complete(false, callback, user_callback, error);
    };
  }

  request_get_file_list(user_callback) {
    this.db_read_request(
      (r) => {
        return r.getAllKeys();
      },
      this.file_list_retrieved.bind(this),
      user_callback,
    );
  }

  request_load_file(filename, suffix, user_callback) {
    this.db_read_request(
      (r) => {
        return r.get(filename);
      },
      this.file_loaded.bind(this),
      user_callback,
    );
  }

  request_save_file(filename, suffix, data, user_callback) {
    this.db_readwrite_request(
      (r) => {
        return r.put({
          filename: filename,
          content: new Uint8Array(data),
          suffix: suffix,
        });
      },
      this.file_saved.bind(this),
      user_callback,
    );
  }

  dir() {
    return this.directory;
  }

  file_list_retrieved(user_callback, success, result) {
    if (success) {
      console.log(`Retrieved file list ${result}`);
      this.directory.clear();
      if (!result) {
      } else {
        for (const r of result) {
          this.directory.add_file(r, "ttf");
        }
      }
    }
    user_callback(success);
  }

  file_loaded(user_callback, success, result) {
    if (success) {
      console.log(`Retrieved file ${result}`);
      user_callback(result);
    } else {
      console.log(`Failed to retrieve file`);
      user_callback(null);
    }
  }

  file_saved(user_callback, success, result) {
    user_callback(success);
  }
}

export class FileSet {
  constructor(storage, prefix) {
    this.storage = storage;
    this.prefix = prefix;
    this.load_dir();
  }

  split_filename(filename) {
    const suffix = filename.split(".").pop();
    if (suffix) {
      const root = filename.slice(0, -suffix.length - 1);
      return [root, suffix];
    } else {
      return null;
    }
  }

  load_dir() {
    this.directory = new Directory();
    const n = this.storage.length;
    const pl = this.prefix.length;
    for (let i = 0; i < n; i++) {
      let k = this.storage.key(i);
      if (k.startsWith(this.prefix)) {
        const f = k.slice(pl);
        const s_r = this.split_filename(f);
        if (s_r) {
          this.directory.add_file(s_r[0], s_r[1]);
        }
      }
    }
  }

  /// Return null if not found
  load_file(root, suffix) {
    let f = this.prefix + root + "." + suffix;
    return this.storage.getItem(f);
  }

  save_file(root, suffix, data) {
    let f = this.prefix + root + "." + suffix;
    this.storage.setItem(f, data);
    this.directory.add_file(suffix, root);
  }

  delete_file(root, suffix) {
    let f = this.prefix + root + "." + suffix;
    this.storage.removeItem(f);
    this.directory.delete_file(root, suffix);
  }

  request_load_file(filename, suffix, user_callback) {
    const data = this.load_file(filename, suffix);
    user_callback(data);
  }

  request_save_file(filename, suffix, data, user_callback) {
    this.save_file(filename, suffix, data);
    user_callback(true);
  }

  dir() {
    return this.directory;
  }
}
