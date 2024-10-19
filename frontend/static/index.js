const showElement = (element) => {
    element.style.visibility = "";
    element.style.maxHeight = "";
    element.style.overflow = "";
};
const hideElement = (element) => {
    element.style.visibility = "hidden";
    element.style.maxHeight = "0";
    element.style.overflow = "hidden";
};
const select = (selector) => document.querySelector(selector);
const selectAll = (selector) => [...document.querySelectorAll(selector)];
selectContent = (identifier) => select(`content[name=${identifier}]`)

const showContent = (identifier) => {
    showElement(select("nav"));
    selectAll("content").map(hideElement);
    showElement(selectContent(identifier));
};

const definePage = (pageName, callback) => () => {
    showContent(pageName);
    callback();
}

const googleUser = () => decodeJwt(localStorage.getItem("credential"));

const showProfile = definePage("profile", () => {
    select("content[name=profile] h2").innerText = googleUser().given_name;
});

const showLogin = definePage("login", () => {
    hideElement(select("nav"));
});

const showEvents = definePage("events", () => {

});

const showBets = definePage("bets", () => {

});

const showCommunity = definePage("community", () => {

});

const showCreateEvent = definePage("create-event", () => {});

const defineMenuItem = (identifier, callback) => {
    const element = select(`nav-item[item-name=${identifier}]`);
    element.addEventListener("click", callback);
};

const addClickHandler = (identifier, callback) => {
    const element = select(`button[name=${identifier}]`);
    element.addEventListener("click", callback);
};

document.addEventListener("DOMContentLoaded", () => {
    new TomSelect('select[name=community]');
    defineMenuItem("bets", showBets);
    defineMenuItem("events", showEvents);
    defineMenuItem("community", showCommunity);
    defineMenuItem("profile", showProfile);
    addClickHandler("show-create-event", showCreateEvent)
    showLogin();
});

const decodeJwt = (token) => JSON.parse(atob(token.split('.')[1]));

function onSignIn(response) {
    localStorage.setItem("credential", response.credential);
    console.log(decodeJwt(response.credential));
    select("content[name=login]").style.display = "none";
    showProfile();
}
