/*  This file is a fairly faithful recreation of Yeongbin Jo's
    python-chromedriver-autoinstaller/src/utils.py.
    For this reason, I have included three functions which
    cannot be used and have added the below line to ignore
    the compiler's warning about them.
 */ 
#![allow(dead_code)]

use is_executable::IsExecutable;
use std::process::{Command, Stdio};
use quick_xml::{Reader, events::Event};
use regex::Regex;
use std::{env, fs, io::{BufReader, Error, ErrorKind, Write}, string};
use std::path::{Path, PathBuf};
use ureq;
use which::which;
use zip::ZipArchive;

/* def get_chromedriver_filename():
    """
    Returns the filename of the binary for the current platform.
    :return: Binary filename
    """
    if sys.platform.startswith('win'):
        return 'chromedriver.exe'
    return 'chromedriver' */

pub fn get_chromedriver_filename() -> &'static str {
	if String::from(env::consts::OS).starts_with("win") {
		"chromedriver.exe"
	} else {
		"chromedriver"
	}
}


/* def get_variable_separator():
    """
    Returns the environment variable separator for the current platform.
    :return: Environment variable separator
    """
    if sys.platform.startswith('win'):
        return ';'
    return ':' */

pub fn get_variable_separator() -> char {
	if String::from(env::consts::OS).starts_with("win") {
		';'
	} else {
		':'
	}
}


/* def get_platform_architecture():
    if sys.platform.startswith('linux') and sys.maxsize > 2 ** 32:
        platform = 'linux'
        architecture = '64'
    elif sys.platform == 'darwin':
        platform = 'mac'
        architecture = '64'
    elif sys.platform.startswith('win'):
        platform = 'win'
        architecture = '32'
    else:
        raise RuntimeError('Could not determine chromedriver download URL for this platform.')
    return platform, architecture */
pub fn get_platform_architecture() -> (&'static str, &'static str) {
	if String::from(env::consts::OS).starts_with("linux") {
		("linux", "64")
	} else if String::from(env::consts::OS).starts_with("darwin") {
		("mac", "64")
	} else if String::from(env::consts::OS).starts_with("win") {
		("win", "32")
	} else {
		panic!("Could not determine chromedriver download URL for this platform.")
	}
}


/* def get_chromedriver_url(version, no_ssl=False):
    """
    Generates the download URL for current platform , architecture and the given version.
    Supports Linux, MacOS and Windows.
    :param version: chromedriver version string
    :param no_ssl: Determines whether or not to use the encryption protocol when downloading the chrome driver.
    :return: Download URL for chromedriver
    """
    if no_ssl:
        base_url = 'http://chromedriver.storage.googleapis.com/'
    else:
        base_url = 'https://chromedriver.storage.googleapis.com/'
    platform, architecture = get_platform_architecture()
    return base_url + version + '/chromedriver_' + platform + architecture + '.zip' */

pub fn get_chromedriver_url(version: &str, no_ssl: bool) -> String {
	let mut url = String::new();
	if no_ssl {
		url.push_str("http://chromedriver.storage.googleapis.com/");
	} else {
		url.push_str("https://chromedriver.storage.googleapis.com/");
	}
	let (platform, architecture) = get_platform_architecture();

	
	url.push_str(version);
	url.push_str("/chromedriver_");
	url.push_str(platform);
	url.push_str(architecture);
	url.push_str(".zip");

	url
}


/* def find_binary_in_path(filename):
    """
    Searches for a binary named `filename` in the current PATH. If an executable is found, its absolute path is returned
    else None.
    :param filename: Filename of the binary
    :return: Absolute path or None
    """
    if 'PATH' not in os.environ:
        return None
    for directory in os.environ['PATH'].split(get_variable_separator()):
        binary = os.path.abspath(os.path.join(directory, filename))
        if os.path.isfile(binary) and os.access(binary, os.X_OK):
            return binary
    return None */

pub fn find_binary_in_path(filename: &str) -> Option<String> {
	if let Ok(path) = env::var("PATH") {
		for p in path.split(get_variable_separator()) {
			let p_str = format!("{}\\{}", p, filename);
			let p_path = Path::new(&p_str);
			if p_path.exists() && p_path.is_executable() {
				return Some(p_str)
			}
		}
	}
	None
}


/* def check_version(binary, required_version):
    try:
        version = subprocess.check_output([binary, '-v'])
        version = re.match(r'.*?([\d.]+).*?', version.decode('utf-8'))[1]
        if version == required_version:
            return True
    except Exception:
        return False
    return False */

pub fn check_version(binary: &str, required_version: &str) -> bool {

	let stdout: String = command_output(binary, vec!["-v"])
    .expect("Error running command.");

	let re = Regex::new(r".*?([\d.]+).*?").unwrap();
	let version = match re.captures(&stdout[..]) {
		None => { return false },
		Some(cap) => {
			cap.get(1).map_or("", |m| m.as_str())
		}
	};

	if version.eq(required_version) {
		return true;
	} else {
		return false;
	}
}


/* def get_chrome_version():
    """
    :return: the version of chrome installed on client
    """
    platform, _ = get_platform_architecture()
    if platform == 'linux':
        path = get_linux_executable_path()
        with subprocess.Popen([path, '--version'], stdout=subprocess.PIPE) as proc:
            version = proc.stdout.read().decode('utf-8').replace('Chromium', '').replace('Google Chrome', '').strip()
    elif platform == 'mac':
        process = subprocess.Popen(['/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', '--version'], stdout=subprocess.PIPE)
        version = process.communicate()[0].decode('UTF-8').replace('Google Chrome', '').strip()
    elif platform == 'win':
        process = subprocess.Popen(
            ['reg', 'query', 'HKEY_CURRENT_USER\\Software\\Google\\Chrome\\BLBeacon', '/v', 'version'],
            stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, stdin=subprocess.DEVNULL
        )
        output = process.communicate()
        if output and output[0] and len(output[0]) > 0:
            version = output[0].decode('UTF-8').strip().split()[-1]
        else:
            process = subprocess.Popen(
                ['powershell', '-command', '$(Get-ItemProperty -Path Registry::HKEY_CURRENT_USER\\Software\\Google\\chrome\\BLBeacon).version'],
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE
            )
            version = process.communicate()[0].decode('UTF-8').strip()
    else:
        return
    return version */

pub fn get_chrome_version() -> Result<String, std::io::Error> {
    let failed_version_msg = "Failed to find Chrome version";

	let (platform, _) = get_platform_architecture();

	if platform.eq("linux") {
		let path = get_linux_executable_path()?;
        match command_output(&path[..], vec!["--version"]) {
            Err(e) => { return Err(e) },
            Ok(s) => {
                return Ok(String::from(s)
                .replace("Chromium", "")
                .replace("Google Chrome", "")
                .trim().to_string());
            }
        }
	} else if platform.eq("mac") {
        match command_output(
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            vec!["--version"]) {
            Err(e) => { return Err(e) },
            Ok(s) => {
                return Ok(s
                .replace("Google Chrome", "")
                .trim().to_string());
            }
        }
    } else if platform.eq("win") {
        let version = match command_output(
            "reg",
            vec!["query",
                "HKEY_CURRENT_USER\\Software\\Google\\Chrome\\BLBeacon",
                "/v",
                "version"]) {
            Ok(output) => {
                // We have run the registry check and it has come back.
                if !(output.eq("")) {
                    let chunks: Vec<&str> = output.trim().split(" ").collect();
                    String::from(chunks[chunks.len() - 1])
                } else {
                    String::from("")
                }                
            },
            Err(_) => { String::from("") }
        };
        if !(version.eq("")) {
            return Ok(version.to_string());
        }
        match command_output(
            "powershell",
            vec!["-command", "$(Get-ItemProperty -Path Registry::HKEY_CURRENT_USER\\Software\\Google\\chrome\\BLBeacon).version"]) {
            Ok(output) => {
                return Ok(String::from(output.trim()));
            },
            Err(e) => { return Err(e) }
        }
    } else {
        return Err(Error::new(ErrorKind::NotFound, failed_version_msg));
    }
}


/* def get_linux_executable_path():
    """
    Look through a list of candidates for Google Chrome executables that might
    exist, and return the full path to first one that does. Raise a ValueError
    if none do.
    :return: the full path to a Chrome executable on the system
    """
    for executable in (
        "google-chrome",
        "google-chrome-stable",
        "google-chrome-beta",
        "google-chrome-dev",
        "chromium-browser",
        "chromium",
    ):
        path = shutil.which(executable)
        if path is not None:
            return path
    raise ValueError("No chrome executable found on PATH") */

pub fn get_linux_executable_path() -> Result<String, std::io::Error> {
	let chrome_names = vec![
    "google-chrome",
    "google-chrome-stable",
    "google-chrome-beta",
    "google-chrome-dev",
    "chromium-browser",
    "chromium"
    ];
    for exec in chrome_names {
        match which(exec) {
            Ok(path) => {
                match path.into_os_string().into_string() {
                    Ok(p) => { return Ok(p) },
                    _ => ()
                }
            },
            _ => ()
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "No chrome executable found on PATH"));
}


/* def get_major_version(version):
    """
    :param version: the version of chrome
    :return: the major version of chrome
    """
    return version.split('.')[0] */

pub fn get_major_version(version: &str) -> String {
    let split: Vec<&str> = version.split('.').collect();
    if split.len() < 2 {
        return version.to_string();
    } else {
        return split[0].to_string();
    }
}


/* def get_matched_chromedriver_version(version, no_ssl=False):
    """
    :param version: the version of chrome
    :return: the version of chromedriver
    """
    if no_ssl:
        doc = urllib.request.urlopen('http://chromedriver.storage.googleapis.com').read()
    else:
        doc = urllib.request.urlopen('https://chromedriver.storage.googleapis.com').read()
    root = elemTree.fromstring(doc)
    for k in root.iter('{http://doc.s3.amazonaws.com/2006-03-01}Key'):
        if k.text.find(get_major_version(version) + '.') == 0:
            return k.text.split('/')[0]
    return */

pub fn get_matched_chromedriver_version(version: &str, no_ssl: bool) -> Result<String, std::io::Error> {
    let doc = match no_ssl {
        true => {"http://chromedriver.storage.googleapis.com"},
        false => {"https://chromedriver.storage.googleapis.com"}
    };

    let resp = ureq::get(doc).call()
    .expect("Failed to get chromedriver version page")
    .into_string()
    .expect("Failed to read chromedriver version page");

    let mut major_version = get_major_version(version);
    major_version.push('.');

    let mut reader = Reader::from_str(&resp[..]);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut found_name = false;

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
        // for triggering namespaced events, use this instead:
        // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
            // for namespaced:
            // Ok((ref namespace_value, Event::Start(ref e)))
                match e.name() {
                    b"Key" => found_name = true,
                    _ => found_name = false,
                }
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let txt = e.unescape_and_decode(&reader).unwrap();
                if found_name {
                    if txt.contains(&major_version) {
                        return Ok(txt.split('/')
                        .collect::<Vec<&str>>()[0]
                        .to_string());
                    }
                    found_name = false;
                }
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }


        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    return Err(Error::new(ErrorKind::NotFound, "Failed to get matching version"));
}


/* def get_chromedriver_path():
    """
    :return: path of the chromedriver binary
    """
    return os.path.abspath(os.path.dirname(__file__)) */

pub fn get_chromedriver_path() -> String {
    return abs_path_string(env::current_dir().unwrap());
}


/* def print_chromedriver_path():
    """
    Print the path of the chromedriver binary.
    """
    print(get_chromedriver_path()) */

pub fn print_chromedriver_path() {
    println!("{}", &get_chromedriver_path()[..])
}


/* def download_chromedriver(path: Optional[AnyStr] = None, no_ssl: bool = False):
    """
    Downloads, unzips and installs chromedriver.
    If a chromedriver binary is found in PATH it will be copied, otherwise downloaded.
    :param str path: Path of the directory where to save the downloaded chromedriver to.
    :param bool no_ssl: Determines whether or not to use the encryption protocol when downloading the chrome driver.
    :return: The file path of chromedriver
    """
    chrome_version = get_chrome_version()
    if not chrome_version:
        logging.debug('Chrome is not installed.')
        return
    chromedriver_version = get_matched_chromedriver_version(chrome_version, no_ssl)
    if not chromedriver_version:
        logging.warning('Can not find chromedriver for currently installed chrome version.')
        return
    major_version = get_major_version(chromedriver_version)

    if path:
        if not os.path.isdir(path):
            raise ValueError(f'Invalid path: {path}')
        chromedriver_dir = os.path.join(
            os.path.abspath(path),
            major_version
        )
    else:
        chromedriver_dir = os.path.join(
            os.path.abspath(os.path.dirname(__file__)),
            major_version
        )
    chromedriver_filename = get_chromedriver_filename()
    chromedriver_filepath = os.path.join(chromedriver_dir, chromedriver_filename)
    if not os.path.isfile(chromedriver_filepath) or \
            not check_version(chromedriver_filepath, chromedriver_version):
        logging.info(f'Downloading chromedriver ({chromedriver_version})...')
        if not os.path.isdir(chromedriver_dir):
            os.makedirs(chromedriver_dir)
        url = get_chromedriver_url(version=chromedriver_version, no_ssl=no_ssl)
        try:
            response = urllib.request.urlopen(url)
            if response.getcode() != 200:
                raise urllib.error.URLError('Not Found')
        except urllib.error.URLError:
            raise RuntimeError(f'Failed to download chromedriver archive: {url}')
        archive = BytesIO(response.read())
        with zipfile.ZipFile(archive) as zip_file:
            zip_file.extract(chromedriver_filename, chromedriver_dir)
    else:
        logging.info('Chromedriver is already installed.')
    if not os.access(chromedriver_filepath, os.X_OK):
        os.chmod(chromedriver_filepath, 0o744)
    return chromedriver_filepath */

pub fn download_chromedriver(path: Option<&str>, no_ssl: bool) -> Result<String, std::io::Error>{
    let chrome_version = &get_chrome_version()
    .expect("Failed to get chrome version")[..];
    let chromedriver_version = &get_matched_chromedriver_version(&chrome_version, no_ssl)
    .expect("Failed to get matched chromedriver version")[..];
    let major_version = &get_major_version(&chromedriver_version[..])[..];

    // finding the directory to place the chromedriver
    let chromedriver_dir = &match path {
        // the function was supplied a directory
        Some(p) => {
            if !(fs::metadata(p)
                .expect("Failed to get metadata for chromedriver directory")
                .is_dir()) {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Invalid path: {}", p)));
            } else {
                String::from(PathBuf::from(p).join(major_version)
                .to_str().unwrap())
            }
        },
        // the function was not supplied a directory
        None => String::from(PathBuf::from(env::current_dir().unwrap())
            .join(major_version).to_str().unwrap()),
    }[..];

    let chromedriver_filename = &get_chromedriver_filename()[..];
    let chromedriver_filepath = &String::from(
        PathBuf::from(chromedriver_dir)
        .join(chromedriver_filename).to_str().unwrap())[..];

    if !(Path::new(chromedriver_filepath).exists())
    || !(check_version(chromedriver_filepath, &chromedriver_version[..])) {
        match fs::metadata(chromedriver_dir) {
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                fs::create_dir_all(chromedriver_dir)
                .expect("Failed to create chromedriver directory")
            },
            Err(e) => return Err(e),
            _ => ()
        }

        let url = &get_chromedriver_url(&chromedriver_version[..], no_ssl)[..];
        let resp = ureq::get(url).call()
        .expect(&format!("Failed to download chromedriver archive: {}", url)[..]);

        let path = Path::new(chromedriver_filename);
        {
            let mut archive = fs::File::create(&path)
            .expect("Failed to create archive file");
            let mut bytes: Vec<u8> = Vec::new();
            resp.into_reader().read_to_end(&mut bytes)
            .expect("Failed to read chromedriver data from the internet");
            archive.write_all(&bytes)
            .expect("Failed to write content to chromedriver archive");
        }

        let archive = fs::File::open(path)
        .expect("Could not open chromedriver archive after downloading");
        let buf_reader = BufReader::new(archive);
        let mut zip = ZipArchive::new(buf_reader)
        .expect("Coult not create zip reader object");
        zip.extract(chromedriver_dir)
        .expect("Failed to extract chromedriver archive");
    } else {
        println!("Chromedriver is already installed.");
    }
    
    Ok(chromedriver_filepath.to_string())
}


/* if __name__ == '__main__':
    print(get_chrome_version())
    print(download_chromedriver(no_ssl=False)) */

// Not implemented here


// Custom functions not found in the original chromedriver-autoupdater by Yeongbin Jo

fn command_output(binary: &str, args: Vec<&str>) -> Result<String, std::io::Error> {

    let mut output = Command::new(binary);
    
    // add arguments
    for arg in args {
        output.arg(arg);
    }

    // tell the OS to record the command's output
    output.stdout(Stdio::piped());
    // execute the command, wait for it to complete,
    // then capture the output
    match output.output() {
        Err(e) => { Err(e) },
        Ok(output) => {
            Ok(string::String::from_utf8(output.stdout).unwrap())
        }
    }
}

pub fn abs_path_string(path: PathBuf) -> String {
    return fs::canonicalize(path)
    .unwrap()         // unwrap canonicalize
    .into_os_string() // this and
    .into_string()    // this convert the PathBuf to a String
    .unwrap();        // unwrap into_string()
}