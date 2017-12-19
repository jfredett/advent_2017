bash watchman/watchman_gen.sh > watchman/watch.sh
chmod a+x watchman/watch.sh
exec watchman/watch.sh
