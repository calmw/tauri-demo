const invoke = window.__TAURI__.invoke

$(function () {

    // 检测是否设置过密码
    $(document).ready(function () {
        invoke("check_pwd",).then((response) => {
            if (response === true) {
                window.location.href = "index.html"
            }
        })
    });

    // 点击确认按钮
    $('#reset_pwd').click(function () {
        let pwd_input_val = $('#pwd_input').val();
        let pwd_input_repeat = $('#pwd_input_repeat').val();
        if (!check_pwd(pwd_input_val, pwd_input_repeat)) {
            return
        }
        invoke("set_pwd", {pwd: pwd_input_val, rePwd: pwd_input_repeat}).then((response) => {
            console.log(response)
            if (response === true) {
                window.location.href="index.html"
            }
        })
    })

    // 检查密码
    function check_pwd(pwd, re_pwd) {
        let result = true;
        if (pwd !== re_pwd) {
            $('#alert_pwd_error .sub_title').text("两次输入密码不一致")
            $('#alert_pwd_error').show()
            result = false
        } else if (pwd.length < 6) {
            $('#alert_pwd_error .sub_title').text("密码长度不能小于6位")
            $('#alert_pwd_error').show()
            result = false
        }
        return result
    }

    // 关闭确认密码弹窗
    $('#alert_pwd_error .close').click(function () {
        $('#alert_pwd_error').hide()
    })
})
