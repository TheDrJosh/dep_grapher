import { createFileRoute, Link } from "@tanstack/react-router";
import { ScrollArea, Tabs } from "radix-ui";
import { type PackageInfo } from "./graph";
import { useForm } from "@tanstack/react-form";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { twJoin } from "tailwind-merge";

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
                                    search={{
                                        source: data.source,
                                        value: data.value,
                                    }}
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
                    <div>git</div>
                </Tabs.Content>
                <Tabs.Content
                    value="cargo"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <div>cargo</div>
                </Tabs.Content>
                <Tabs.Content
                    value="npm"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <div>npm</div>
                </Tabs.Content>
                <Tabs.Content
                    value="jsr"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <div>jsr</div>
                </Tabs.Content>
                <Tabs.Content
                    value="pypi"
                    className="grow rounded-b-4xl bg-primary p-5 outline-none dark:bg-primary-dark"
                >
                    <div>pypl</div>
                </Tabs.Content>
            </Tabs.Root>
        </>
    );
}

function LocalProjectForm() {
    const form = useForm({
        defaultValues: {
            path: "",
        },
        onSubmit: ({ value }) => {},
    });

    return (
        <>
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    form.handleSubmit();
                }}
            >
                <div>
                    <form.Field
                        name="path"
                        validators={{
                            onChangeAsync: async ({ value }) => {
                                try {
                                    await invoke("is_path_valid", {
                                        path: value,
                                    });
                                    return undefined;
                                } catch (e) {
                                    if (typeof e === "string") {
                                        console.log(e);

                                        return e;
                                    } else {
                                        return "unknown";
                                    }
                                }
                            },
                        }}
                    >
                        {(field) => (
                            <div>
                                <label htmlFor={field.name}>
                                    Enter path to project
                                </label>
                                <div>
                                    <input
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={(e) =>
                                            field.handleChange(e.target.value)
                                        }
                                        className={twJoin(
                                            "w-96 rounded bg-white p-1 text-black focus:outline",
                                            field.state.meta.errors.length
                                                ? "border-3 border-red-600"
                                                : "border-2 border-backround dark:border-backround-dark"
                                        )}
                                        autoComplete="false"
                                        autoCorrect="false"
                                    />

                                    <button
                                        onClick={() => {
                                            open({
                                                multiple: false,
                                                directory: true,
                                            }).then((dir) => {
                                                if (dir) {
                                                    field.setValue((_) => dir);
                                                }
                                            });
                                        }}
                                        className=""
                                    >
                                        Browse
                                    </button>
                                </div>
                                <div className="h-8">
                                    {field.state.meta.errors.length ? (
                                        <em>
                                            {field.state.meta.errors.join(",")}
                                        </em>
                                    ) : null}
                                </div>
                            </div>
                        )}
                    </form.Field>
                    {/* Select what project(s) in that workspace you want to see*/}
                </div>
                <button type="submit" className="">
                    Submit
                </button>
            </form>
        </>
    );
}
