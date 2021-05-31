enum TrafficLight {
    Green,
    Red,
    Yellow,
}

type Minute = usize;
trait LightDuration {
    fn duration(&self) -> Minute;
}

impl LightDuration for TrafficLight {
    fn duration(&self) -> Minute {
        match self {
            TrafficLight::Green => 3,
            TrafficLight::Red => 5,
            TrafficLight::Yellow => 4
        }
    }
}