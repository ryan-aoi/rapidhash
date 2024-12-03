import pytest

import rapidhash


@pytest.mark.parametrize(
    "key, seed, gt",
    [
        (b"hello world", None, 17498481775468162579),
        (b"hello world", 0, 6388527444622164108),
        (b"hello,", 0, 17861179120578160190),
        (b"hello, world!", 1, 4668653575921246457),
        (b"Hello, world!", 1, 7739271034020981250),
        (b"hello world! ", 2, 10327466050248778708),
    ],
)
def test_short_cases(key, seed, gt):
    assert rapidhash.rapidhash(key, seed) == gt


@pytest.mark.parametrize(
    "key, seed, gt",
    [
        (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
nulla pariatur. Excepteur sint occaecat cupidatat non proident, \
sunt in culpa qui officia deserunt mollit anim id est laborum.",
            None,
            1221157664313218070,
        ),
        (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. \
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat \
nulla pariatur. Excepteur sint occaecat cupidatat non proident, \
sunt in culpa qui officia deserunt mollit anim id est laborum.",
            1,
            805527774265126985,
        ),
        (
            "Sed ut perspiciatis, unde omnis iste natus error sit voluptatem accusantium \
doloremque laudantium, totam rem aperiam eaque ipsa, quae ab illo inventore \
veritatis et quasi architecto beatae vitae dicta sunt, explicabo. Nemo enim ipsam \
voluptatem, quia voluptas sit, aspernatur aut odit aut fugit, sed quia consequuntur \
magni dolores eos, qui ratione voluptatem sequi nesciunt, neque porro quisquam est, \
qui dolorem ipsum, quia dolor sit, amet, consectetur, adipisci velit, sed quia non \
numquam eius modi tempora incidunt, ut labore et dolore magnam aliquam quaerat \
voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis \
suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum \
iure reprehenderit, qui in ea voluptate velit esse, quam nihil molestiae \
consequatur, vel illum, qui dolorem eum fugiat, quo voluptas nulla pariatur? At \
vero eos et accusamus et iusto odio dignissimos ducimus, qui blanditiis praesentium \
voluptatum deleniti atque corrupti, quos dolores et quas molestias excepturi sint, \
obcaecati cupiditate non provident, similique sunt in culpa, qui officia deserunt \
mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est \
et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio, \
cumque nihil impedit, quo minus id, quod maxime placeat, facere possimus, omnis \
voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut \
officiis debitis aut rerum necessitatibus saepe eveniet, ut et voluptates \
repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a \
sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut \
perferendis doloribus asperiores repellat.",
            None,
            16702286806359783625,
        ),
    ],
)
def test_long_cases(key, seed, gt):
    assert rapidhash.rapidhash(key.encode(), seed) == gt
