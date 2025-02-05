import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";

export const Route = createRootRoute({
    component: () => (
        <>
            <div className="flex h-dvh flex-col bg-backround dark:bg-backround-dark dark:text-white">
                <div className="flex items-center gap-2 bg-primary p-2 dark:bg-primary-dark">
                    <Link
                        to="/"
                        className="text-4xl font-bold tracking-tighter text-accent shadow-orange-300 hover:[text-shadow:_0_0_3px_var(--tw-shadow-color)] dark:shadow-orange-700"
                    >
                        Dep Grapher
                    </Link>
                    <div className="flex-1"></div>
                    <Link
                        to="/about"
                        className="hover:underline [&.active]:font-bold"
                    >
                        About
                    </Link>
                </div>
                <div className="flex-grow">
                    <Outlet />
                </div>
                <TanStackRouterDevtools />
                <ReactQueryDevtools initialIsOpen={false} />
            </div>
        </>
    ),
});
