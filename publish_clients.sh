sudo chown -R $(id -u):$(id -g) clients

create_repo_if_not_exists(){
    repo_name=$1
    repo_exists=$(gh repo list --json name --jq ".[] | select(.name==\"$1\") | length")
    if [ -z "$repo_exists" ]; then
        echo "Creating repo $repo_name"
        gh repo create $repo_name --public
    fi
}

cd clients

for f in *; do
    if [ -d "$f" ]; then
        # Will not run if no directories are available
        create_repo_if_not_exists "notifier-sdk-$f"
        cd $f
        chmod +x git_push.sh
        ./git_push.sh
        cd ..
    fi
done