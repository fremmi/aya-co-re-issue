# Reproduces https://github.com/aya-rs/aya/issues/722: task_struct offsets
# baked into vmlinux.rs on one kernel don't match a different kernel's real
# layout, since aya-ebpf's bpf_probe_read_kernel does a raw offset read with
# no BTF CO-RE relocation.
Vagrant.configure("2") do |config|
  config.vm.box = "debian/bookworm64"
  config.vm.provider :libvirt do |lv|
    lv.memory = 2048
    lv.cpus = 2
  end
  config.vm.synced_folder ".", "/vagrant", type: "rsync"
end
