var original = document.location.hash;
var queryString = "?" + original.substring(1, original.length);
window.location.replace("http://localhost:7684/token" + queryString);
