const invoke = window.__TAURI__.invoke

$(function () {
    // 初始化左侧菜单
    $(".menu_iterm").eq(0).addClass("active")
    $(".menu_iterm").eq(0).siblings().removeClass("active")

    // 左侧菜单切换
    $(".menu_iterm").click(function () {
        $(this).addClass("active")
        $(this).siblings().removeClass("active")
    });
})