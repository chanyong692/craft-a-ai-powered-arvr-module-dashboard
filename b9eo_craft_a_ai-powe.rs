// Define a data model for the AI-powered AR/VR module dashboard

mod models {
    pub struct User {
        id: i32,
        name: String,
        email: String,
    }

    pub struct ARVRModule {
        id: i32,
        name: String,
        description: String,
        module_type: ModuleType,
        ai_model: AiModel,
    }

    pub enum ModuleType {
        AR,
        VR,
    }

    pub struct AiModel {
        id: i32,
        name: String,
        model_type: AiModelType,
        accuracy: f64,
    }

    pub enum AiModelType {
        ObjectDetection,
        ImageClassification,
        SpeechRecognition,
    }

    pub struct Dashboard {
        id: i32,
        name: String,
        description: String,
        modules: Vec<ARVRModule>,
        users: Vec<User>,
    }

    pub struct ModuleInstance {
        id: i32,
        module_id: i32,
        user_id: i32,
        start_time: i64,
        end_time: i64,
        data: Vec<ModuleData>,
    }

    pub struct ModuleData {
        id: i32,
        instance_id: i32,
        data_type: DataType,
        value: String,
    }

    pub enum DataType {
        Image,
        Audio,
        Text,
    }
}