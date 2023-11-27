const invoke = window.__TAURI__.invoke

$(function () {

    // 导入私钥
    $(".import_btn").click(function () {
        let import_key_val = $('#import_key_input').val();
        // add_private_key_to_config
        invoke("add_private_key_to_config",{key:import_key_val}).then((response) => {
            console.log(response)
            // if (response === false) {
            //     window.location.href = "set.html"
            // }
        })
    });

    // 底部返回
    $("#footer .back").click(function () {
        window.history.back()
    });

})