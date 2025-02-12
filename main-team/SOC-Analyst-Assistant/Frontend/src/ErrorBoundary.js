import React from "react";

class ErrorBoundary extends React.Component {
    constructor(props) {
        super(props);
        this.state = { hasError: false };
    }

    static getDerivedStateFromError(error) {
        // Update state so the next render shows the fallback UI.
        return { hasError: true };
    }

    componentDidCatch(error, errorInfo) {
        // Log the error details for debugging
        console.error("Error in component:", error, errorInfo);
    }

    render() {
        if (this.state.hasError) {
            // Render fallback UI with a custom message
            return (
                <div style={{ color: "red", margin: "10px 0" }}>
                    {this.props.fallbackMessage || "An error occurred in this section."}
                </div>
            );
        }

        return this.props.children;
    }
}

export default ErrorBoundary;
