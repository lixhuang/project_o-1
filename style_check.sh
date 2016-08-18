#!/usr/bin/env sh
panic_if_not_empty() {
    if [ -n "$1" ]
    then
        echo "$1"
        echo "$2"
        exit 1
    fi
}

tabs_in_src=$(grep '\t' -Prn -m 1 src/)
panic_if_not_empty "$tabs_in_src" "Style check fails, tabs are evil :("

spaces_in_src=$(grep '[[:space:]]\+$' -rn -m 1 src/)
panic_if_not_empty "$spaces_in_src" "Style check fails, trailing spaces at EOL :("

comments_in_src=$(grep '/\*' -rn --include=*.rs src/)
panic_if_not_empty "$comments_in_src" "Style check fails, use // or /// instead of /**/ :("

comments_in_src=$(grep '^[[:space:]]*//' -vrn --include=*.rs src/ | grep "//")
panic_if_not_empty "$comments_in_src" "Style check fails, use // in a sperate line :("

fns_in_src=$(grep '//' -vrn --include=*.rs src/ | grep 'fn ' | grep 'fn.\+{' -v)
panic_if_not_empty "$fns_in_src" "Style check fails, '{' for 'fn' missing at EOL :("

fors_in_src=$(grep '//' -vrn --include=*.rs src/ | grep 'for ' | grep 'for.\+{' -v)
panic_if_not_empty "$fors_in_src" "Style check fails, '{' for 'for' missing at EOL :("

ifs_in_src=$(grep '//' -vrn --include=*.rs src/ | grep 'if ' | grep 'if.\+{' -v)
panic_if_not_empty "$ifs_in_src" "Style check fails, '{' for 'if' missing at EOL :("

curlys_in_src=$(grep '[^[:space:]]{' -rn --include=*.rs src/ | grep '"[^{]*?{[^}]*?}.*?"' -vP)
panic_if_not_empty "$curlys_in_src" "Style check fails, sticky curly found :("
