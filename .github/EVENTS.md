## Main Events

Present in: [`src/main/ipc-events/index.ts`](/src/main/ipc-events/index.ts)

- `close-app`: Calls `app.quit()` and closes the close-app.
- `set-title`: Takes an id argument and sets the window title.

## Create Project Events

Present in [`src/main/ipc-events/create.ts`](/src/main/ipc-events/create.ts)

- `open-directory`: Opens the dialog for choosing directory for project files.
  - **Replies:**
    - `opened-directory`: Returns if cancelled or not, path to dir (if not cancelled) and if selected dir is empty (if not cancelled)
- `open-yml`: This is for selecting a pre-existing project.
  - **Replies:**
    - `opened-yml`: As of now just returns dir data (cancelled, filePaths).
- `create-project-in-dir`: Accepts project title (name), project description and project path in an object. Tries creating the default folder paths `/assets/minecraft/textures/block` and a `project.yml` file in the provided path. Then saves these info with a generated UUID in the app's `config.json` file.
  - **Replies:**
    - `error-create`: If an error occurs while trying to create the project dir, it returns with the error message.
    - `created`: If successful returns the project id, so that user can be navigated to that page (`/:id`).
- `selected-path-in-config`: Checks if the passed path arg is present in the app's `config.json` file.
  - **Replies:**
    - `selected-path-is-in-config`: Verifies the yml and returns {boolean, message}.
- `get-yml-data`: Accepts the path to `project.yml` file and parses the data.
  - **Replies:**
    - `yml-data`: Returns the parsed data if exists, else false.
- `add-existing-project`: Accepts the yml path, project title and description then tries to save the project to the app's config file and update the `project.yml` file if changes.
  - **Replies:**
    - `created`: Returns only if project addition is successful, returns project ID.
    - `error-create`: Returns a string with error message on failure.

## Project Events

Present in [`src/main/ipc-events/project.ts`](/src/main/ipc-events/project.ts)

- `get-my-projects`: Used for fetching the config data from the app's `config.json` file.
  - **Replies:**
    - `my-projects`: Returns the config data.
- `get-project-data-with-id`: Accepts an id. Fetches the project file data from the app's `config.json` and then filters with id.
  - **Replies:**
    - `project-data-reply`: Returns false if no project file with that id is found, else returns the project data if found.
- `open-in-folder`: Accepts path and opens the path in OS default files software.
- `delete-project`: Accepts ID and calls the function to remove the project.
  - **Replies:**
    - `deleted-project:` Returns an object with message for toast and error indicator.
- `get-project-file-tree`: Accepts project path and calls the tree builder function.
  - **Replies:**:
    - `project-file-tree`: If file tree is found, returns file tree, else redirects user to home page.
