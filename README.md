# Build Your Own Virtual Machine with /dev/kvm and Rust!

This repository contains the source code from the 2019 Confreaks talk, [Build your own virtual machine with /dev/kvm and Rust!](https://www.youtube.com/watch?v=A_diEEpAfpM&t=484s). The demonstration illustrates using Rust, kvm-ioctls, and vm-memory to construct a simple virtual machine that performs the addition of two numbers and outputs the result to stdout.

## Prerequisites

Before diving into this guide, ensure you have:
- A Linux system with KVM enabled.
- The Rust programming language installed on your system.

### Checking KVM Support and Permissions on Your System

Before proceeding with running the example, it's crucial to verify both the support for KVM on your system and that your user account has the necessary permissions to interact with KVM.

1. **Verify KVM Support**:
    - Check for CPU virtualization support by running `egrep -c '(vmx|svm)' /proc/cpuinfo` in the terminal. A non-zero output signifies hardware virtualization support.
    - Ensure the KVM module is active by executing `lsmod | grep kvm`. You should see `kvm_intel` or `kvm_amd` listed.

        - Load KVM for Intel CPUs with `sudo modprobe kvm_intel`.
        - Load KVM for AMD CPUs with `sudo modprobe kvm_amd`.

    - Confirm the existence of the `/dev/kvm` device file with `ls -l /dev/kvm`. Its presence indicates KVM is operational.

2. **Check User Permissions for /dev/kvm**:
    - The current user must have read and write access to `/dev/kvm`. Check this by running `ls -l /dev/kvm`, which shows the permissions and the user/group ownership.
    - Typically, `/dev/kvm` belongs to the `root` user and the `kvm` group. Ensure your user is part of the `kvm` group by running `sudo usermod -aG kvm $USER`, then log out and back in for the changes to take effect. You can verify your group membership with `groups $USER`.

If `/dev/kvm` does not exist or you can't load the KVM module, ensure that virtualization is enabled in your BIOS/UEFI settings.

## Running the Example

After ensuring your system is properly set up, follow these steps to run the virtual machine example:

1. **Clone the Repository**: Use `git clone` to download this repository to your machine.

2. **Build the Project**: Navigate to the project directory and execute `cargo build` to compile the source code.

3. **Execute the Virtual Machine**: Launch the compiled binary with `cargo run`. If set up correctly, you'll see the output from the virtual machine performing the addition.

## Troubleshooting

- If encountering KVM permission errors, confirm your user is in the `kvm` group or try running the binary with `sudo`.

- Ensure you're using the latest Rust version for compatibility; update Rust with `rustup update`.

For more detailed code explanations, refer to the [original Confreaks talk](https://www.youtube.com/watch?v=A_diEEpAfpM&t=484s).

Enjoy building and running your virtual machines with Rust and KVM!

