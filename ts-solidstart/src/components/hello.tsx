interface HelloProps {
    myProp: string;
    otherProp: number;
    Component: Element;
}

export default function Hello({myProp, otherProp, Component, children}: HelloProps) {


    return <>
        <p>Hello world from this component {myProp} {otherProp > 10 ? "hello world" : "Other world"}</p>
    </>
}