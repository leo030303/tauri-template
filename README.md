# Tauri template
A Tauri template that uses all Rust, setup for Android and Linux, using Leptos for the frontend and Bootstrap for CSS. To get started:
* Clone the repo
* Follow the instructions [here](https://v2.tauri.app/distribute/apk-sign/) to create the keys necessary for Android APK signing
* Create a file called key.properties for the required info, and place it in `tauri-template/src-tauri/gen/android`
* Replace every instance of `leo030303` with your Github username and `tauritemplate` with the name of your app, including in file and folder names. You can use the commands 
  * `find . -type f -exec sed -i 's/tauritemplate/your_app_name/g' {} +`
  * `find . -type f -exec sed -i 's/leo030303/your_username/g' {} +`
  * `find . -type d -name "tauritemplate" -exec rename 's/tauritemplate/your_app_name/' {} \;`
  * `find . -type d -name "leo030303" -exec rename 's/leo030303/your_username/' {} \;`
* Run `cargo tauri dev` and you should be good to go!
