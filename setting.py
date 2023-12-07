import json
import pathlib

def main():
    code_workspace_path = 'atcoder.code-workspace'
    with open(code_workspace_path, 'r') as f:
        code_workspace = json.load(f)
    print(code_workspace['folders'])
    code_workspace['folders'] = [{"path": "."}]
    root_dir = '.'
    root_dir = pathlib.Path(root_dir)
    rust_paths = [
        {
            "path": path.parent.as_posix(),
            # "name" : path.parent.as_posix().replace('/', '-'),
        # "name" : path.parent.name+'test',
        } for path in root_dir.glob(pattern='**/Cargo.toml')
    ]
    code_workspace['folders'] += rust_paths
    print(code_workspace)
    with open(code_workspace_path, 'w') as f:
        json.dump(code_workspace, f, indent=4)
    
if __name__ == '__main__':
    main()