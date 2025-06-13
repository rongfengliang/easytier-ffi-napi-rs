use crate::bindings;
use crate::bindings::KeyValuePair;
pub fn parse_config(conf: &str) -> Result<i32, String> {
    // 解析配置文件
    use std::ffi::CString;
    let c_conf = CString::new(conf).map_err(|e| e.to_string())?;
    let result = unsafe { bindings::parse_config(c_conf.as_ptr()) };
    // 打印配置内容
    println!("Parsed config: {:?}", result);
    if result < 0 {
        return Err("Failed to parse config".to_string());
    }
    Ok(0)
}

pub fn run_network_service(conf: &str) -> Result<(), String> {
    // 启动网络服务
    use std::ffi::CString;
    let c_conf = CString::new(conf).map_err(|e| e.to_string())?;
    let result = unsafe { bindings::run_network_instance(c_conf.as_ptr()) };

    // 打印网络服务状态
    println!("Network service status: {:?}", result);
    if result < 0 {
        return Err("Failed to run network service".to_string());
    }
    Ok(())
}


pub fn collect_network_infos() -> Result<Vec<KeyValuePair>, String> {
    let max_length = 10;
    let mut infos: Vec<KeyValuePair> =  vec![KeyValuePair {
        key: std::ptr::null_mut(),
        value: std::ptr::null_mut(),
    }; max_length];

    let result = unsafe {
        bindings::collect_network_infos(infos.as_mut_ptr(), max_length)
    };
    let mut result_network: Vec<KeyValuePair> = Vec::new();
    if result >= 0 {

        println!("成功收集到 {} 条网络信息", result);
        for i in 0..result as usize {
            let info = &infos[i];
            result_network.push(KeyValuePair { key:info.key, value:info.value });
        }
        Ok(result_network)
    } else {
        println!("收集失败，错误码：{}", result);
        Err("Failed to collect network infos".to_string())
    }
}