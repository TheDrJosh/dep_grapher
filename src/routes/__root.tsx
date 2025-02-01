import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";

export const Route = createRootRoute({
    component: () => (
        <>
            <div className="flex h-dvh flex-col dark:bg-neutral-700 dark:text-white">
                <div className="flex items-center gap-2 bg-neutral-100 p-2 dark:bg-neutral-600">
                    <Link
                        to="/"
                        className="text-4xl font-bold tracking-tighter text-orange-500 shadow-orange-700 hover:[text-shadow:_0_0_3px_var(--tw-shadow-color)]"
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
            </div>
        </>
    ),
});
