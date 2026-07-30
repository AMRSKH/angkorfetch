class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/AMRSKH/angkorfetch"
  # No explicit `version` stanza: Homebrew scans it from the vX.Y.Z path segment
  # of the URLs below, and `brew audit --strict` rejects restating it.
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.1/angkorfetch-macos-aarch64.tar.gz"
      sha256 "63d34b6237bb07bb67a42c0ec2f372387728c0bab3208b31666ac387ef9dc7b7"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.1/angkorfetch-macos-x86_64.tar.gz"
      sha256 "2d9998ed1efb13db8269a783fabc71cb892c939959fdfcfc476b7b6f412b1e78"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.1/angkorfetch-linux-aarch64.tar.gz"
      sha256 "2e285780e36519b8fd59c6fda6fa6903d5e96f7f98bfd4080ca0b0ed58c05fc6"
    end

    on_intel do
      url "https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.1/angkorfetch-linux-x86_64.tar.gz"
      sha256 "37c03558e0a0a59374f642d514d2d22f85439d23a550dead01aa96e1f0655397"
    end
  end

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{bin}/angkorfetch --version")
  end
end
