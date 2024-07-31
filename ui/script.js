const { invoke } = window.__TAURI__.tauri

old = "Hello World";

function btn_onclick() {
	invoke("hello", { name: "World" }).then((res) => {
        old = window.tutle.innerHTML;
		window.tutle.innerHTML = res;
    })
}
