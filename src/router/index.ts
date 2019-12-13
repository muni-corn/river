import Vue from "vue";
import VueRouter, { Route } from "vue-router";
import Home from "../views/Home.vue";
import HTTPService from "@/services/HTTPService";

Vue.use(VueRouter);

const routes = [
    {
        path: "/",
        name: "home",
        component: Home
    },
    {
        path: "",
        component: () => import("../views/Auth.vue"),
        children: [
            {
                path: "register",
                name: "register",
                component: () => import("../components/Register.vue")
            },
            {
                path: "login",
                name: "login",
                component: () => import("../components/Login.vue")
            }
        ]
    }
];

const router = new VueRouter({
    mode: "history",
    base: process.env.BASE_URL,
    routes
});

async function authGuard(to: Route, _: Route, next: Function) {
    if (
        to.path == "/login" ||
        to.path == "/register" ||
        (await HTTPService.isAuthenticated())
    ) {
        next();
    } else {
        next({ name: "login" });
    }
}

router.beforeEach(authGuard);

export default router;
