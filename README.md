# rust-fsm-example
Examples of rust state machine design patterns

## 什么是状态机
状态机无处不在，像我们常用的 tcp、http、regexp 等等本质上都是状态机。

状态机是由状态和状态间的转换构成的。

拿红绿灯来举个简单的例子，红绿灯会处于 3 个状态：红、绿、黄，这几个状态之间有确定的转换路径：

```text
+-----------+      +------------+      +---------+
|   Green   +----->+   Yellow   +----->+   Red   |
+-----+-----+      +------------+      +----+----+
      ^                                     |
      |                                     |
      +-------------------------------------+
```

如果用 rust 来写的话，我可能会这样实现：

```Rust
use std::thread::sleep;
use core::borrow::Borrow;
use std::time::Duration;

// 把红绿灯看成一个状态机，状态转换过程如下：
//+-----------+      +------------+      +---------+
//|   Green   +----->+   Yellow   +----->+   Red   |
//+-----+-----+      +------------+      +----+----+
//      ^                                     |
//      |                                     |
//      +-------------------------------------+

#[derive(Debug)]
enum TrafficLightState {
    Red { waiting_time: std::time::Duration },
    Yellow { waiting_time: std::time::Duration },
    Green { waiting_time: std::time::Duration },
}

struct TrafficLight {
    state: TrafficLightState,
}

fn change_light(mut state: &TrafficLightState) -> TrafficLightState {
    match state {
        TrafficLightState::Green { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Yellow { waiting_time: std::time::Duration::new(10, 0) }
        },
        TrafficLightState::Red { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) }
        },
        TrafficLightState::Yellow { waiting_time } => {
            sleep(*waiting_time);
            TrafficLightState::Red { waiting_time: std::time::Duration::new(60, 0) }
        }
    }
}

fn main() {
    let mut state_machine = TrafficLight{
        state: TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) }
    };

    loop {
        println!("{:?}", state_machine.state);
        state_machine.state = change_light(&state_machine.state)
    }
}
```

初始为绿色状态，60s 后切换为黄色状态，10s 后切换为红色状态，60s 后又切换回绿色，如此循环往复。

这段代码虽然实现了我们的需求，但是并不是很漂亮。除了存在一些重复的代码，更严重的问题是状态的变换完全暴露给了外部，这意味着外部可以做任意的状态切换，比如红色 -> 黄色，黄色 -> 绿色 等等，这并不是我们希望的行为。而且从职责分离的角度来看，状态的切换也应该是由一个状态机根据当前的输入和一些环境条件自行决定的，这些行为不需要也不应该让外部知道。

现在我们再来描述一下我们想要实现的状态机：
1. 在某一时刻，状态机处于一个确定的状态
2. 每个状态都可以有其相关的取值
3. 状态之间的切换需要有确定的语义
4. 允许一定程度的状态共享
5. 只允许发生声明过的状态切换，不允许出现不明路径的状态变换
6. 从一个状态切换到另一个状态后，就不应该有任何依赖前一个状态的地方了
7. 错误信息应该容易让人理解

## 将状态控制在状态机内部
```Rust
impl TrafficLight {
    fn new() -> Self {
        TrafficLight {
            state: TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) }
        }
    }

    fn change_light(&mut self) {
        self.state = match self.state {
            TrafficLightState::Green { waiting_time } => {
                sleep(waiting_time);
                TrafficLightState::Yellow { waiting_time: std::time::Duration::new(10, 0) }
            },
            TrafficLightState::Red { waiting_time } => {
                sleep(waiting_time);
                TrafficLightState::Green { waiting_time: std::time::Duration::new(60, 0) }
            },
            TrafficLightState::Yellow { waiting_time } => {
                sleep(waiting_time);
                TrafficLightState::Red { waiting_time: std::time::Duration::new(60, 0) }
            }
        }
    }
}
```

我们使用 rust 的关键字 **impl** 为类型 TrafficLight 关联了 new 方法和 change_light 方法。之后外部就可以直接调用 new 来进行初始化了，而修改状态只需要调用 change_light 就可以了。

## 使用泛型和类型转换
面对复杂状态的转换时，我们最好能把所有的状态和变换路径都描述出来。我们可以利用泛型来描述多种状态，并使用 **From** 和 **Into** 来描述状态间的变换。

```Rust
use std::thread::sleep;
use core::borrow::Borrow;
use std::time::Duration;
use std::cmp::Ordering::Greater;

// 把红绿灯看成一个状态机，状态转换过程如下：
//+-----------+      +------------+      +---------+
//|   Green   +----->+   Yellow   +----->+   Red   |
//+-----+-----+      +------------+      +----+----+
//      ^                                     |
//      |                                     |
//      +-------------------------------------+

#[derive(Debug)]
struct Red {
    wait_time: Duration
}

impl Red {
    fn new() -> Self {
        Red{
            wait_time: Duration::new(60, 0)
        }
    }
}

#[derive(Debug)]
struct Green {
    wait_time: std::time::Duration
}

impl Green {
    fn new() -> Self {
        Green{
            wait_time: Duration::new(60, 0)
        }
    }
}

#[derive(Debug)]
struct Yellow {
    wait_time: std::time::Duration
}

impl Yellow {
    fn new() -> Self {
        Yellow{
            wait_time: Duration::new(10, 0)
        }
    }
}

#[derive(Debug)]
struct TrafficLight<TLS> {
    state: TLS,
}

impl TrafficLight<Green> {
    fn new() -> Self {
        TrafficLight {
            state: Green::new(),
        }
    }
}

impl From<TrafficLight<Green>> for TrafficLight<Yellow> {
    fn from(green: TrafficLight<Green>) -> TrafficLight<Yellow> {
        println!("last state is {:?}", green);
        sleep(green.state.wait_time);
        TrafficLight {
            state: Yellow::new(),
        }
    }
}

impl From<TrafficLight<Yellow>> for TrafficLight<Red> {
    fn from(yellow: TrafficLight<Yellow>) -> TrafficLight<Red> {
        println!("last state is {:?}", yellow);
        sleep(yellow.state.wait_time);
        TrafficLight {
            state: Red::new(),
        }
    }
}

impl From<TrafficLight<Red>> for TrafficLight<Green> {
    fn from(red: TrafficLight<Red>) -> TrafficLight<Green> {
        println!("last state is {:?}", red);
        sleep(red.state.wait_time);
        TrafficLight {
            state: Green::new(),
        }
    }
}

enum TrafficLightWrapper {
    Red(TrafficLight<Red>),
    Green(TrafficLight<Green>),
    Yellow(TrafficLight<Yellow>),
}

impl TrafficLightWrapper {
    fn new() -> Self {
        TrafficLightWrapper::Green(TrafficLight::new())
    }
    fn step(mut self) -> Self {
        match self {
            TrafficLightWrapper::Green(green) => TrafficLightWrapper::Yellow(green.into()),
            TrafficLightWrapper::Yellow(yellow) => TrafficLightWrapper::Red(yellow.into()),
            TrafficLightWrapper::Red(red) => TrafficLightWrapper::Green(red.into())
        }
    }
}

fn main() {
    let mut state_machine = TrafficLightWrapper::new();

    loop {
        state_machine = state_machine.step();
    }
}
```

##总结
把上面的代码抽象一下，我们就能得到状态机设计模式，之后解决类似问题，就可以直接利用这个模型了：
1. 定义全部状态
2. 定义一个状态容器，可以使用泛型以及实现相应的 from 方法来完成状态之间的切换
3. 定义枚举类型描述全部的可选状态
4. 初始化状态机，调用相应的驱动方法，启动状态机

嗯，Rust 真香！

## Reference
* [Pretty State Machine pattern](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
