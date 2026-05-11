#!/usr/bin/sh
# vim: set sw=4 et ai :

unset -v IFS

toplevel=$(git rev-parse --show-toplevel)
printf 'cd %s\n' "$toplevel"
cd "$toplevel" || return 10
(set -x; pwd)

reports_dir=reports
report_file=tree-reports.txt

[ -d "$reports_dir" ] || (set -x; mkdir -p "$reports_dir") || return 11
[ -d "$reports_dir" ] || return 12
printf 'cd %s\n' "$reports_dir"
cd "$reports_dir" || return 13
(set -x; pwd)

print_args () (
    while :; do
        if [ ! ${1+x} ]; then break; fi
        printf ' %s' $(
            /usr/bin/env -i ARG="${1:-}" /usr/bin/sh -c 'export -p'    \
            | /usr/bin/grep -P '^\s*(export|declare\s+-x)\s+ARG='      \
            | /usr/bin/sed -En -e 's/^(export|declare -x)[^=]+=(.+)/\2/p'
        )
        shift
    done
    echo
)

tree_report () {
    e="${1:-}"
    [ -n "$e" ] || return 20
    shift

    set -- cargo tree --offline -e "$e" -f '{p} {f}' "$@"
    print_args "$@"
    ec=127
    {
        printf '\n++=================================================================================================|'
        printf '\n++=================================================================================================|'
        printf '\n||'
        printf '\n|| '
        print_args "$@"
        printf '||'
        printf '\n++=================================================================================================|'
        printf '\n++=================================================================================================|'
        printf '\n\n'
        "$@"
        ec=$?
    } >>"$report_file" 2>&1
    printf '\nexit code: %s\n' "$ec" >>"$report_file"
    printf '    exit code: %s\n' "$ec"
}

[ -f "$report_file" ] && (set -x; rm "$report_file")
[ -f "$report_file" ] && return 30

set -o errexit
tree_report normal
tree_report features
tree_report normal --duplicates

return 0
