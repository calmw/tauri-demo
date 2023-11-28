const invoke = window.__TAURI__.invoke

$(function () {
    // 初始化左侧菜单
    $(".menu_iterm").eq(0).addClass("active")
    $(".menu_iterm").eq(0).siblings().removeClass("active")

    // 左侧菜单切换
    $(".menu_iterm").click(function () {
        let menu_index=$(this).index()
        console.log(menu_index)
        if (menu_index===0){
            $('#frame').attr("src","./user_center/account.html")
        }else if (menu_index===1){
            $('#frame').attr("src","./user_center/network.html")
        }else if (menu_index===2){
            $('#frame').attr("src","./user_center/finance.html")
        }else if (menu_index===3){
            $('#frame').attr("src","./user_center/transfer.html")
        }
        $(this).addClass("active")
        $(this).siblings().removeClass("active")
    });
})