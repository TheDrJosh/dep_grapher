import { createFileRoute, Link } from "@tanstack/react-router";

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
            <div className="flex-1 rounded-4xl bg-neutral-600">
                <span>Recent</span>
                <ul></ul>
            </div>
        </div>
    );
}

function LocalButton() {
    return (
        <>
            <button
                className="flex-1 rounded-4xl bg-orange-600 text-4xl hover:bg-orange-500"
                onClick={() => {
                    console.log("hi");
                }}
            >
                Local
            </button>
        </>
    );
}
