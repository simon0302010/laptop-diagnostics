if [[ "$(tty)" == "/dev/tty1" ]]; then
    chmod a+x /root/laptop-diagnostics
    /root/laptop-diagnostics
fi
