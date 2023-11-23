// When using the Tauri API npm package:
// import { invoke } from '@tauri-apps/api/tauri'
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke

$(function () {
    $('#unlock').click(function () {
        console.log($('#pwd_input').val())
        invoke("unlock", {password: $('#pwd_input').val()}).then((response) => {
            // window.header.innerHTML = response
            console.log(response)
        })
    })

    // 打开无法解锁弹窗
    $('#unlock_error').click(function () {
       $('#alert_cant_not_unlock').show()
    })

    // 关闭无法解锁弹窗
    $('#alert_cant_not_unlock .close').click(function () {
       $('#alert_cant_not_unlock').hide()
    })
})

//invoke('greet', { name: 'World2234' })
//     // `invoke` returns a Promise
//     .then((response) => {
//         // window.header.innerHTML = response
//         console.log(response)
//     })