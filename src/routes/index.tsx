import { createFileRoute, Link } from "@tanstack/react-router";
import { ScrollArea, Tabs } from "radix-ui";
import { type PackageInfo } from "./graph";

import { LocalProjectForm } from "../components/local_form";
import { RegistryProjectForm } from "../components/registry_form";

export const Route = createFileRoute("/")({
    component: Index,
});

function Index() {
    return (
        <div className="flex h-full gap-4 p-4">
            <ProjectSelectForm />
            <RecentList />
        </div>
    );
}

function RecentList() {
    const sample_data: PackageInfo[] = [
        {
            source: "npm",
            value: "leftpad",
            type: "nodejs",
        },
    ];

    return (
        <>
            <div className="flex flex-1 flex-col rounded-4xl bg-primary dark:bg-primary-dark">
                <span className="self-center p-2 text-2xl">Recent</span>
                <ScrollArea.Root>
                    <ScrollArea.Viewport>
                        <div className="p-4">
                            {sample_data.map((data) => (
                                <Link
                                    key={data.value + data.source}
                                    to="/graph"
                                    search={data}
                                    className="hover:underline"
                                >
                                    leftpad{" "}
                                    <span className="text-neutral-400">
                                        npm
                                    </span>
                                </Link>
                            ))}
                        </div>
                    </ScrollArea.Viewport>
                    <ScrollArea.Scrollbar orientation="vertical"></ScrollArea.Scrollbar>
                    <ScrollArea.Corner className="" />
                </ScrollArea.Root>
            </div>
        </>
    );
}

function ProjectSelectForm() {
    return (
        <>
            <Tabs.Root className="flex flex-3 flex-col" defaultValue="local">
                <Tabs.List className="flex shrink-0 border-b border-backround dark:border-backround-dark">
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="local"
                    >
                        Local
                    </Tabs.Trigger>
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="git"
                    >
                        Git
                    </Tabs.Trigger>
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="cargo"
                    >
                        Cargo
                    </Tabs.Trigger>
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="npm"
                    >
                        Npm
                    </Tabs.Trigger>
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="jsr"
                    >
                        Jsr
                    </Tabs.Trigger>
                    <Tabs.Trigger
                        className="flex h-10 flex-1 cursor-default items-center justify-center bg-primary px-5 text-sm leading-none outline-none select-none first:rounded-tl-4xl last:rounded-tr-4xl hover:text-accent data-[state=active]:text-accent data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current dark:bg-primary-dark"
                        value="pypi"
                    >
                        PyPI
                    </Tabs.Trigger>
                </Tabs.List>
                <Tabs.Content
                    value="local"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <LocalProjectForm />
                </Tabs.Content>
                <Tabs.Content
                    value="git"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    git
                </Tabs.Content>
                <Tabs.Content
                    value="cargo"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <RegistryProjectForm registry_type="cargo" />
                </Tabs.Content>
                <Tabs.Content
                    value="npm"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <RegistryProjectForm registry_type="npm" />
                </Tabs.Content>
                <Tabs.Content
                    value="jsr"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <RegistryProjectForm registry_type="jsr" />
                </Tabs.Content>
                <Tabs.Content
                    value="pypi"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <RegistryProjectForm registry_type="pypi" />
                </Tabs.Content>
            </Tabs.Root>
        </>
    );
}
