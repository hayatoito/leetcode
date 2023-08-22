self=${0:a}

link() {
  mkdir -p ~/.leetup
  ln -snf $self/config.json ~/.leetup
}

leetup_install() {
  cd ~/tmp
  curl -LO https://github.com/dragfire/leetup/releases/latest/download/leetup-x86_64-unknown-linux-musl.tar.gz
  tar xvf leetup-x86_64-unknown-linux-musl.tar.gz
  mv leetup ~/src/bin
  ls -l ~/src/bin/leetup
}

login() {
  # Open leetcode site in a browser at first, then run this.
  # Use: --port=10006 for remote foward.
  # leetup user --cookie "$(my-browser $@ leetcode-cookie)"
  my-browser $@ leetcode-cookie | leetup user --cookie
  echo "See ~/.leetup/*.log, which are Key-Value-Store"
}

pick() {
  # Usage:
  # > mm pick <id>
  # > mm pick --lang python3 <id>

  # Wrapper for 'leetup pick <id>'
  # 1. Pick problem (generate a file)
  # 2. Rename the generated file. e.g. "two-sum.rs" -> "0001-two-sum.rs"
  # 3. Clean up the file
  # 3. Open the file with emacsclient

  cd $self/src/bin
  # Example output of 'leetup pick ID'
  # > leetup pick 1
  # Generated: /usr/local/google/home/hayato/src/leetcode/two-sum.rs
  local file=$(leetup pick $@ | my-ansi-color-remover | awk '{print $2}')

  # Left-Pad zero to problem_id.  e.g. "12" -> "0012"
  local id=$@[-1]
  id=${(l:4::0:)id}

  local dst="${file:h}/${id}-${file:t}"
  echo "$dst"
  if [[ -f $dst ]]; then
    echo "$dst already exists."
    rm $file
  else
    mv $file $dst

    # For Rust: Extract solution function name
    local function_name=$(grep -o -P '^ *pub fn \K([a-z_]+)' $dst)

    local sed_args=(

        # Replace __func__ with function name
        -e "s/__func__/${function_name}/"
        # Remove lines
        -e '/^.*@leetup=custom.*$/d'
        -e '/^.*@leetup=inject.*$/d'
        # For Python3 typehint: List[x] -> list[x]
        -e 's/List\[/list\[/g'
        # For cpp
        -e 's/vector</std::vector</g'
        -e 's/\(string /(std::string /g'
        -e 's/, string /, std::string /g'
        # Remove problem description (for Rust and C++)
        -e '/^\/?\*/d'
        # Remove problem description (for Pythong)
        -e '/^# [^@]/d'
        -e '/^# *$/d'
        )
    sed -i -E $sed_args $dst

    # For Rust: Add todo!()
    # e.g.
    # Before:
    #     pub fn longest_palindrome(s: String) -> String {
    #
    #     }
    # // @leetup=code
    #
    # After:
    #     pub fn longest_palindrome(s: String) -> String {
    #         todo!()
    #     }
    # // @leetup=code
    sd -f m ' *\n    \}\n}\n// @leetup=code' '        todo!()\n    }\n}\n// @leetup=code' $dst

    # For Python3: Add 'pass'
    # e.g.
    # Before:
    #     def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
    #
    # @leetup=code
    #
    # After:
    #     def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
    #         pass
    #
    # @leetup=code
    #
    sd -f m ': *\n *\n# @leetup=code' ':\n        pass\n\n# @leetup=code' $dst
  fi

  my-open-file $dst
}
