import {
    type FieldApi,
    type ReactFormExtendedApi,
    useForm,
} from "@tanstack/react-form";
import { open } from "@tauri-apps/plugin-dialog";
import { twJoin } from "tailwind-merge";
import { useQuery } from "@tanstack/react-query";
import { debug } from "@tauri-apps/plugin-log";
import { commands, type Project } from "../bindings";

interface FormValues {
    path: string;
    selected_project: Project | null;
}

export function LocalProjectForm() {
    const form = useForm<FormValues>({
        defaultValues: {
            path: "",
            selected_project: null,
        } as const,
        onSubmit: ({ value }) => {
            debug(JSON.stringify(value));
        },
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
                <PathSelector form={form} />
                <form.Subscribe
                    selector={(state) =>
                        [
                            state.values.path,
                            state.fieldMeta.path
                                ? state.fieldMeta.path.errors.length
                                : 1,
                        ] satisfies [string, number]
                    }
                >
                    {([path, errors_length]) => (
                        <ProjectSelector
                            form={form}
                            path={path}
                            error_length={errors_length}
                        />
                    )}
                </form.Subscribe>

                <button type="submit" className="">
                    Submit
                </button>
            </form>
        </>
    );
}

function PathSelector(params: {
    form: ReactFormExtendedApi<FormValues, undefined>;
}) {
    const { form } = params;

    return (
        <form.Field
            name="path"
            validators={{
                onChangeAsync: async ({ value }) => {
                    const res = await commands.isPathValid(value);

                    if (res.status == "ok") {
                        return undefined;
                    } else {
                        return res.error;
                    }
                },
                onMount: () => "NotAbsolute",
            }}
            listeners={{
                onChange: () => {
                    form.setFieldValue("selected_project", null);
                },
            }}
        >
            {(field) => (
                <div>
                    <label htmlFor={field.name}>Enter path to project</label>
                    <div>
                        <input
                            name={field.name}
                            value={field.state.value}
                            onBlur={field.handleBlur}
                            onChange={(e) => field.handleChange(e.target.value)}
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
                            <em className="text-red-500">
                                {field.state.meta.errors.join(",")}
                            </em>
                        ) : null}
                    </div>
                </div>
            )}
        </form.Field>
    );
}

function ProjectSelector(params: {
    form: ReactFormExtendedApi<FormValues, undefined>;
    path: string;
    error_length: number;
}) {
    const { form, path, error_length } = params;

    const { data, error } = useQuery({
        queryKey: ["project_names", path],
        queryFn: async () => {
            const res = await commands.getProjectsInDir(path);

            if (res.status == "ok") {
                return res.data;
            } else {
                throw res.error;
            }
        },

        enabled: error_length == 0,
    });

    return (
        <div>
            <form.Field
                name="selected_project"
                validators={{
                    onChange: ({ value }) => {
                        if (value === null) {
                            return "Must Select a Project";
                        }
                    },
                }}
            >
                {(field) => (
                    <div>
                        {error ? (
                            <em className="text-red-500">{error.toString()}</em>
                        ) : data ? (
                            <>
                                {data.map((project) => (
                                    <ProjectSelectBox
                                        project={project}
                                        field={field}
                                        key={
                                            project.name +
                                            "|" +
                                            project.project_type
                                        }
                                    />
                                ))}

                                <div className="h-8">
                                    {field.state.meta.errors.length ? (
                                        <em className="text-red-500">
                                            {field.state.meta.errors.join(",")}
                                        </em>
                                    ) : null}
                                </div>
                            </>
                        ) : (
                            <></>
                        )}
                    </div>
                )}
            </form.Field>
        </div>
    );
}

function ProjectSelectBox(params: {
    project: Project;
    field: FieldApi<
        FormValues,
        "selected_project",
        undefined,
        undefined,
        Project | null
    >;
}) {
    const { project, field } = params;
    return (
        <div>
            <input
                type="radio"
                id={project.name + "|" + project.project_type}
                name={field.name}
                value={JSON.stringify(project)}
                onBlur={field.handleBlur}
                onChange={(e) => {
                    field.handleChange(
                        JSON.parse(e.target.value) as Project
                    );
                }}
            />
            <label htmlFor={project.name + "|" + project.project_type}>
                {project.name}{" "}
                <span className="text-neutral-400">{project.project_type}</span>
            </label>
        </div>
    );
}
