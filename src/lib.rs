mod utils;

use std::{env, path::Path};


/* def install(cwd: bool = False, path: Optional[AnyStr] = None, no_ssl: bool = False):
    """
    Appends the directory of the chromedriver binary file to PATH.
    :param cwd: Flag indicating whether to download to current working directory. If the `cwd` is True, then path argument will be ignored.
    :param path: Specify the path where the Chrome driver will be installed. If the `cwd` value is True, this value is ignored.
    :param no_ssl: Determines whether or not to use the encryption protocol when downloading the chrome driver.
    :return: The file path of chromedriver
    """
    if cwd:
        path = os.getcwd()
    chromedriver_filepath = utils.download_chromedriver(path, no_ssl)
    if not chromedriver_filepath:
        logging.debug('Can not download chromedriver.')
        return
    chromedriver_dir = os.path.dirname(chromedriver_filepath)
    if 'PATH' not in os.environ:
        os.environ['PATH'] = chromedriver_dir
    elif chromedriver_dir not in os.environ['PATH']:
        os.environ['PATH'] = chromedriver_dir + utils.get_variable_separator() + os.environ['PATH']
    return  */

pub fn install(cwd: bool, path: Option<&str>, no_ssl: bool) -> Result<String, std::io::Error> {

    // Where will we be installing the chromedriver?
    // This evaluates to a String (which may be "") to resolve ownership issues
    // regarding the abs_path_string() call.
    let path_to_use: String = match cwd {
        // cwd == true, so use the current working directory.
        true => utils::abs_path_string(
            env::current_dir().unwrap())
            .to_string(),
        // cwd == false, so use whatever the supplied "path" is.
        false => match path {
            Some(p) => p.to_string(),
            None => "".to_string(),
        },
    };

    // run download_chromedriver()
    let chromedriver_filepath = match utils::download_chromedriver(
        match path_to_use.as_str() {
            "" => None,
            _ => Some(path_to_use.as_str()),
        }, no_ssl) {
        Ok(used_path) => used_path,
        Err(e) => return Err(e)
    };

    let chromedriver_dir = Path::new(&chromedriver_filepath)
    .parent().unwrap()
    .to_str().unwrap();
    
    if let Ok(path) = env::var("PATH") {
        // path is set
        let path = path.to_string();

        if !(path.contains(chromedriver_dir)) {
            // path does not include the chromedriver.
            // add it
            let mut new_path = path.to_string();
            new_path.push_str(&String::from(utils::get_variable_separator()));
            new_path.push_str(&chromedriver_dir);

            env::set_var("PATH", new_path);
        }
        
    } else {
        // path is not set
        env::set_var("PATH", chromedriver_dir);
    }

    Ok(chromedriver_filepath.to_string())
}


/* def get_chrome_version():
    """
    Get installed version of chrome on client
    :return: The version of chrome
    """
    return utils.get_chrome_version() */

pub fn get_chrome_version() -> Result<String, std::io::Error> {
    utils::get_chrome_version()
}



#[cfg(test)]
#[test]
fn test() {
    println!("Tests: ");
    /* println!("Chromedriver filename is {}", utils::get_chromedriver_filename());
    println!("Variable separator is {}", utils::get_variable_separator());
    let (platform, arch) = utils::get_platform_architecture();
    println!("Platform architecture is {} {}", platform, arch);
    println!("Gotten chromedriver URL is {}", utils::get_chromedriver_url("1.0.0", false));
    println!("Binary found: {}", utils::find_binary_in_path("explorer.exe").unwrap());
    println!("Version check: {}", utils::check_version("O:\\Brendan\\Scripts\\rust\\version_printer\\target\\release\\version_printer.exe", "1.0.0"));
    let chrome_version = utils::get_chrome_version().unwrap();
    println!("Chrome version is {}", chrome_version);
    match utils::get_linux_executable_path() {
        Ok(s) => { println!("Linux exec path: {}", s) },
        Err(_) => { println!("Error finding Linux executable path.")}
    }
    println!("Major Chrome version is {}", utils::get_major_version(&chrome_version[..]));
    let matched_version = utils::get_matched_chromedriver_version(&chrome_version, false).unwrap();
    println!("Matched chromedriver version is {}", matched_version);
    print!("Printing chromedriver path: ");
    utils::print_chromedriver_path();
    print!("Downloading chromedriver... ");
    match utils::download_chromedriver(None, false) {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }; */
    install(false, None, false).unwrap();
}