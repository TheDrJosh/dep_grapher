import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

export const Route = createFileRoute("/")({
    component: Index,
});

function Index() {
    return (
        <div className="flex h-full gap-4 p-4">
            <LocalButton />
            <Link
                to="/remote"
                className="flex-1 content-center rounded-4xl bg-orange-600 text-center text-4xl hover:bg-orange-500"
            >
                Remote
            </Link>
            <RecentList />
        </div>
    );
}

function LocalButton() {
    const [path, setPath] = useState<string | null>(null);

    const navigate = useNavigate();

    useEffect(() => {
        console.log(path);
        if (path != null) {
            navigate({
                to: "/graph",
                search: {
                    type: "local",
                    path: path,
                },
            });
        }
    }, [path]);

    return (
        <>
            <button
                className="flex-1 rounded-4xl bg-orange-600 text-4xl hover:bg-orange-500"
                onClick={() => {
                    open({ multiple: false, directory: true }).then((dir) => {
                        setPath(dir);
                    });
                }}
            >
                Local
            </button>
        </>
    );
}

function RecentList() {
    return (
        <>
            <div className="flex flex-1 flex-col rounded-4xl bg-neutral-100 dark:bg-neutral-600">
                <span className="self-center p-2 text-2xl">Recent</span>
                <hr className="text-neutral-700" />
                <ul className="p-4">
                    <li>
                        <Link
                            to="/graph"
                            search={{
                                type: "npm",
                                path: "leftpad",
                            }}
                            className="hover:underline"
                        >
                            leftpad{" "}
                            <span className="text-neutral-300 dark:text-neutral-400">
                                npm
                            </span>
                        </Link>
                    </li>
                </ul>
            </div>
        </>
    );
}
