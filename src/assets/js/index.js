const invoke = window.__TAURI__.invoke

$(function () {

    // 检测是否设置过密码
    $(document).ready(function () {
        invoke("check_pwd",).then((response) => {
            if (response === false) {
                window.location.href = "set.html"
            }
        })
    });

    // 点击解锁按钮
    $('#unlock').click(function () {
        invoke("unlock", {password: $('#pwd_input').val()}).then((response) => {
            if (response === true) {
                // private_key_exits
                invoke("private_key_exits", {}).then((response) => {
                    if (response === true) {
                        window.location.href = "user_center.html"
                    } else {
                        window.location.href = "import_private_key.html"
                    }
                })
            } else {
                $('#alert_pwd_error .sub_title').text("密码错误")
                $('#alert_pwd_error').show()
            }
        })
    })

    // 关闭解锁密码错误弹窗
    $('#alert_pwd_error .close').click(function () {
        $('#alert_pwd_error').hide()
    })

    // 打开无法解锁弹窗
    $('#unlock_error').click(function () {
        $('#alert_cant_not_unlock').show()
    })

    // 关闭无法解锁弹窗
    $('#alert_cant_not_unlock .close').click(function () {
        $('#alert_cant_not_unlock').hide()
    })

    // 确认无法解锁弹窗
    $('#alert_cant_not_unlock .ok').click(function () {
        invoke("reset_pwd", {}).then((response) => {
            if (response === true) {
                window.location.href = "set.html"
            }
        })
    })

})