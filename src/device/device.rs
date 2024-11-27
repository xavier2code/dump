// Device info struct
pub struct Device {
    d_name:String,
    d_type:String,
    d_date:String,
    d_number:String,
    sales_information: SalesInformation,
    use_information: UseInformation,
    d_location:String,
}

// sales information
pub struct SalesInformation {
    s_date:String,
    c_name:String,
    c_address:String,
}

// use information
pub struct UseInformation {
    u_on_time:String,
    u_off_time:String,
    u_health:String,
    u_power:String,
    u_model:String,
    u_times:String,
}

impl Device {
    pub fn show() {

    }

    pub fn lock() {

    }

    pub fn notify() {

    }

    pub fn limit() {

    }

    pub fn alert() {

    }
}